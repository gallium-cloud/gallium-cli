use crate::task_common::error::TaskError;
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use comfy_table::Table;
use comfy_table::presets::NOTHING;
use indicatif::HumanDuration;
use serde::Serialize;
use serde_json::Value;
use snafu::prelude::*;

const MISSING: &str = "-";
const COLUMN_GAP: u16 = 2;

#[derive(clap::Args)]
pub struct OutputArgs {
    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    output: OutputFormat,
}

impl OutputArgs {
    pub fn is_json(&self) -> bool {
        matches!(self.output, OutputFormat::Json)
    }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Clone, Copy)]
pub enum Format {
    Text,
    Age,
    Boolean,
}

pub struct Column {
    pub header: &'static str,
    pub pointer: &'static str,
    pub format: Format,
}

pub struct Field {
    pub label: &'static str,
    pub pointer: &'static str,
    pub format: Format,
}

pub fn render_list<T: Serialize>(
    response: &T,
    items: &str,
    columns: &[Column],
    args: &OutputArgs,
) -> Result<(), TaskError> {
    let response = to_value(response)?;
    let text = match args.output {
        OutputFormat::Text => list_text(&response, items, columns, Utc::now())?,
        OutputFormat::Json => to_json(&response)?,
    };
    println!("{text}");
    Ok(())
}

pub fn render_result<T: Serialize>(
    response: &T,
    message: &str,
    args: &OutputArgs,
) -> Result<(), TaskError> {
    let text = match args.output {
        OutputFormat::Text => message.to_string(),
        OutputFormat::Json => to_json(&to_value(response)?)?,
    };
    println!("{text}");
    Ok(())
}

pub enum View {
    List {
        items: &'static str,
        columns: &'static [Column],
    },
    Detail {
        fields: &'static [Field],
    },
    Lines {
        items: &'static str,
        pointer: &'static str,
    },
}

pub struct Section {
    pub name: Option<&'static str>,
    pub title: Option<&'static str>,
    pub view: View,
}

pub struct Report<'a> {
    args: &'a OutputArgs,
    responses: Vec<Response>,
}

struct Response {
    sections: Vec<&'static Section>,
    value: Value,
}

impl Response {
    fn name(&self) -> Option<&'static str> {
        self.sections.first().and_then(|section| section.name)
    }
}

impl<'a> Report<'a> {
    pub fn new(args: &'a OutputArgs) -> Self {
        Self {
            args,
            responses: Vec::new(),
        }
    }

    pub fn add<T: Serialize>(
        &mut self,
        section: &'static Section,
        response: &T,
    ) -> Result<(), TaskError> {
        self.add_views(&[section], response)
    }

    pub fn add_views<T: Serialize>(
        &mut self,
        sections: &[&'static Section],
        response: &T,
    ) -> Result<(), TaskError> {
        self.responses.push(Response {
            sections: sections.to_vec(),
            value: to_value(response)?,
        });
        Ok(())
    }

    pub fn print(self) -> Result<(), TaskError> {
        let text = match self.args.output {
            OutputFormat::Text => self.text(Utc::now())?,
            OutputFormat::Json => to_json(&self.json())?,
        };
        if !text.is_empty() {
            println!("{text}");
        }
        Ok(())
    }

    fn text(&self, now: DateTime<Utc>) -> Result<String, TaskError> {
        let mut parts = Vec::new();
        for response in &self.responses {
            for section in &response.sections {
                let body = match &section.view {
                    View::List { items, columns } => {
                        list_text(&response.value, items, columns, now)?
                    }
                    View::Detail { fields } => detail_text(&response.value, fields, now),
                    View::Lines { items, pointer } => lines_text(&response.value, items, pointer)?,
                };
                parts.push(match section.title {
                    Some(title) => format!("{title}:\n{body}"),
                    None => body,
                });
            }
        }
        Ok(parts.join("\n\n"))
    }

    fn json(&self) -> Value {
        match self.responses.as_slice() {
            [response] if response.name().is_none() => response.value.clone(),
            responses => responses
                .iter()
                .map(|response| {
                    (
                        response.name().unwrap_or_default().to_string(),
                        response.value.clone(),
                    )
                })
                .collect(),
        }
    }
}

fn to_value<T: Serialize>(response: &T) -> Result<Value, TaskError> {
    serde_json::to_value(response).whatever_context("serialize API response")
}

fn to_json(response: &Value) -> Result<String, TaskError> {
    serde_json::to_string_pretty(response).whatever_context("serialize API response")
}

fn list_text(
    response: &Value,
    items: &str,
    columns: &[Column],
    now: DateTime<Utc>,
) -> Result<String, TaskError> {
    let Some(items) = response.pointer(items).and_then(Value::as_array) else {
        whatever!("API response has no list at {items}");
    };
    let mut table = new_table();
    table.set_header(columns.iter().map(|column| column.header));
    for item in items {
        table.add_row(
            columns
                .iter()
                .map(|column| cell(item.pointer(column.pointer), column.format, now)),
        );
    }
    Ok(table_text(table))
}

fn lines_text(response: &Value, items: &str, pointer: &str) -> Result<String, TaskError> {
    let Some(items) = response.pointer(items).and_then(Value::as_array) else {
        whatever!("API response has no list at {items}");
    };
    Ok(items
        .iter()
        .map(|item| match item.pointer(pointer) {
            Some(Value::String(text)) => text.clone(),
            Some(other) => other.to_string(),
            None => String::new(),
        })
        .collect::<Vec<_>>()
        .join("\n"))
}

fn detail_text(response: &Value, fields: &[Field], now: DateTime<Utc>) -> String {
    let mut table = new_table();
    for field in fields {
        table.add_row([
            format!("{}:", field.label),
            cell(response.pointer(field.pointer), field.format, now),
        ]);
    }
    table_text(table)
}

fn new_table() -> Table {
    let mut table = Table::new();
    table.load_style(NOTHING);
    table
}

fn table_text(mut table: Table) -> String {
    for column in table.column_iter_mut() {
        column.set_padding((0, COLUMN_GAP));
    }
    table
        .lines()
        .map(|line| line.trim_end().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn cell(value: Option<&Value>, format: Format, now: DateTime<Utc>) -> String {
    match (value, format) {
        (None | Some(Value::Null), _) => MISSING.to_string(),
        (Some(Value::String(text)), Format::Age) => age(text, now).unwrap_or_else(|| text.clone()),
        (Some(Value::String(text)), _) => text.clone(),
        (Some(Value::Bool(true)), Format::Boolean) => "yes".to_string(),
        (Some(Value::Bool(false)), Format::Boolean) => "no".to_string(),
        (Some(other), _) => other.to_string(),
    }
}

fn age(timestamp: &str, now: DateTime<Utc>) -> Option<String> {
    let then = DateTime::parse_from_rfc3339(timestamp).ok()?;
    let elapsed = now.signed_duration_since(then).to_std().unwrap_or_default();
    Some(format!("{:#} ago", HumanDuration(elapsed)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    static STATUS: Section = Section {
        name: Some("status"),
        title: None,
        view: View::Detail {
            fields: &[Field {
                label: "Online",
                pointer: "/online",
                format: Format::Boolean,
            }],
        },
    };

    static APPS: Section = Section {
        name: Some("apps"),
        title: Some("Apps"),
        view: View::List {
            items: "/items",
            columns: &[Column {
                header: "NAME",
                pointer: "/name",
                format: Format::Text,
            }],
        },
    };

    static UNNAMED: Section = Section {
        name: None,
        title: None,
        view: View::Detail { fields: &[] },
    };

    const ARGS: OutputArgs = OutputArgs {
        output: OutputFormat::Text,
    };

    #[test]
    fn sections_print_in_order_with_titles() {
        let mut report = Report::new(&ARGS);
        report.add(&STATUS, &json!({ "online": true })).unwrap();
        report
            .add(&APPS, &json!({ "items": [{ "name": "wiki" }] }))
            .unwrap();

        let text = report.text(Utc::now()).unwrap();

        assert_eq!(text, "Online:  yes\n\nApps:\nNAME\nwiki");
    }

    #[test]
    fn json_output_keys_by_section() {
        let mut report = Report::new(&ARGS);
        report.add(&STATUS, &json!({ "online": true })).unwrap();
        report.add(&APPS, &json!({ "items": [] })).unwrap();

        assert_eq!(
            report.json(),
            json!({ "status": { "online": true }, "apps": { "items": [] } })
        );
    }

    #[test]
    fn json_output_one_section() {
        let mut report = Report::new(&ARGS);
        report.add(&UNNAMED, &json!({ "online": true })).unwrap();

        assert_eq!(report.json(), json!({ "online": true }));
    }

    static LOG: Section = Section {
        name: None,
        title: None,
        view: View::Lines {
            items: "/events",
            pointer: "/text",
        },
    };

    #[test]
    fn log_line_formatting() -> Result<(), TaskError> {
        let mut report = Report::new(&ARGS);
        report.add(
            &LOG,
            &json!({ "events": [{ "text": "started  worker 1" }, { "text": "" }, { "text": "ready" }] }),
        )?;
        assert_eq!(report.text(Utc::now())?, "started  worker 1\n\nready");

        let mut empty = Report::new(&ARGS);
        empty.add(&LOG, &json!({ "events": [] }))?;
        assert_eq!(empty.text(Utc::now())?, "");
        Ok(())
    }

    static HOSTS: Section = Section {
        name: None,
        title: None,
        view: View::List {
            items: "",
            columns: &[Column {
                header: "NAME",
                pointer: "/name",
                format: Format::Text,
            }],
        },
    };

    #[test]
    fn top_level_array() -> Result<(), TaskError> {
        let mut report = Report::new(&ARGS);
        report.add(
            &HOSTS,
            &json!([{ "name": "edge-syd-01" }, { "name": "lab" }]),
        )?;

        assert_eq!(report.text(Utc::now())?, "NAME\nedge-syd-01\nlab");
        assert_eq!(
            report.json(),
            json!([{ "name": "edge-syd-01" }, { "name": "lab" }])
        );
        Ok(())
    }
}
