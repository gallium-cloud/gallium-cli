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

pub fn render_detail<T: Serialize>(
    response: &T,
    fields: &[Field],
    args: &OutputArgs,
) -> Result<(), TaskError> {
    let response = to_value(response)?;
    let text = match args.output {
        OutputFormat::Text => detail_text(&response, fields, Utc::now()),
        OutputFormat::Json => to_json(&response)?,
    };
    println!("{text}");
    Ok(())
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
