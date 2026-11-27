use crate::args::GlobalArguments;
use crate::helpers::document::read_document;
use crate::helpers::output::{OutputArgs, render_result};
use crate::task_common::error::{AppSpecInvalidSnafu, TaskError};
use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Group, Level, Origin, Renderer, Snippet};
use gallium_api_client::apis::app_spec_api;
use gallium_api_client::models::{AppSpecValidation, Issue, Severity};
use snafu::ensure;
use std::io::IsTerminal;
use std::ops::Range;
use std::path::PathBuf;

#[derive(clap::Subcommand)]
pub(crate) enum AppSpecCommand {
    /// Validate an AppSpec file
    Validate(ValidateArguments),
}

#[derive(clap::Args)]
pub(crate) struct ValidateArguments {
    /// File containing the AppSpec to validate
    #[arg(short = 'f', long, value_name = "PATH")]
    file: PathBuf,
    #[command(flatten)]
    output: OutputArgs,
}

pub(crate) async fn app_spec_main(
    global_args: &GlobalArguments,
    command: AppSpecCommand,
) -> Result<(), TaskError> {
    match command {
        AppSpecCommand::Validate(args) => validate(global_args, args).await,
    }
}

async fn validate(global_args: &GlobalArguments, args: ValidateArguments) -> Result<(), TaskError> {
    let document = read_document(&args.file).await?;
    let api_client = global_args.build_unauthed_api_client()?;
    let validation = app_spec_api::validate_app_spec(api_client.config(), &document).await?;

    let renderer = if std::io::stdout().is_terminal() {
        Renderer::styled().decor_style(DecorStyle::Unicode)
    } else {
        Renderer::plain()
    };
    let path = args.file.display().to_string();
    let text = report(&validation, &document, &path, &renderer);
    render_result(&validation, &text, &args.output)?;

    ensure!(validation.valid, AppSpecInvalidSnafu { path: args.file });
    Ok(())
}

fn report(
    validation: &AppSpecValidation,
    document: &str,
    path: &str,
    renderer: &Renderer,
) -> String {
    let mut issues: Vec<&Issue> = validation.issues.iter().collect();
    issues.sort_by_key(|issue| issue.line);
    let mut parts: Vec<String> = issues
        .into_iter()
        .map(|issue| renderer.render(&[diagnostic(issue, document, path)]))
        .collect();
    if !validation.valid {
        return parts.join("\n\n") + "\n";
    }
    parts.push(match validation.issues.len() {
        0 => format!("{path} validated successfully"),
        1 => format!("{path} validated successfully (1 warning)"),
        warnings => format!("{path} validated successfully ({warnings} warnings)"),
    });
    parts.join("\n\n")
}

fn diagnostic<'a>(issue: &'a Issue, document: &'a str, path: &'a str) -> Group<'a> {
    let title = level(&issue.severity)
        .primary_title(issue.message.as_str())
        .id(issue.code.as_str());
    let line = issue.line;
    match line.and_then(|line| line_span(document, line)) {
        Some(entry) => {
            let annotation = if issue.path.is_empty() {
                AnnotationKind::Primary.span(entry.start..entry.start)
            } else {
                AnnotationKind::Primary
                    .span(entry)
                    .label(issue.path.as_str())
            };
            title.element(Snippet::source(document).path(path).annotation(annotation))
        }
        None => {
            let origin = Origin::path(path);
            title.element(match line.and_then(|line| usize::try_from(line).ok()) {
                Some(line) => origin.line(line),
                None => origin,
            })
        }
    }
}

fn level(severity: &Severity) -> Level<'static> {
    match severity {
        Severity::Warning => Level::WARNING,
        Severity::Error | Severity::UnknownValue(_) => Level::ERROR,
    }
}

fn line_span(document: &str, line: u32) -> Option<Range<usize>> {
    let index = usize::try_from(line).ok()?.checked_sub(1)?;
    let mut start = 0;
    for (current, text) in document.split('\n').enumerate() {
        if current == index {
            let content = text.trim_end();
            let indent = content.len() - content.trim_start().len();
            return (!content.is_empty()).then(|| start + indent..start + content.len());
        }
        start += text.len() + 1;
    }
    None
}
