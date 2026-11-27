use crate::task_common::error::TaskError;
use snafu::ResultExt;
use yaml_rust2::{Yaml, YamlEmitter};

const DOCUMENT_START: &str = "---\n";

pub fn string(value: &str) -> Yaml {
    Yaml::String(value.to_string())
}

pub fn map<const N: usize>(entries: [(&str, Yaml); N]) -> Yaml {
    Yaml::Hash(
        entries
            .into_iter()
            .map(|(key, value)| (string(key), value))
            .collect(),
    )
}

pub fn document(root: Yaml) -> Result<String, TaskError> {
    let mut text = String::new();
    YamlEmitter::new(&mut text)
        .dump(&root)
        .whatever_context::<_, TaskError>("serialize YAML document")?;
    let text = text.strip_prefix(DOCUMENT_START).unwrap_or(&text);
    Ok(format!("{text}\n"))
}
