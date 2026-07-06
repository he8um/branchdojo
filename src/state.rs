use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};

pub const STATE_FILE: &str = ".branchdojo.json";
pub const TOOL_NAME: &str = "branchdojo";
pub const SCHEMA_VERSION: &str = "0.1.0";
pub const VALIDATION_POLICY: &str = "final-state";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BranchDojoState {
    pub tool: String,
    pub schema_version: String,
    pub exercise: String,
    pub created_at: String,
    pub expected_branch: String,
    pub expected_files: Vec<String>,
    pub validation_policy: String,
}

impl BranchDojoState {
    pub fn new(exercise: impl Into<String>, expected_files: Vec<String>) -> Self {
        Self {
            tool: TOOL_NAME.to_string(),
            schema_version: SCHEMA_VERSION.to_string(),
            exercise: exercise.into(),
            created_at: "2026-07-07T00:00:00Z".to_string(),
            expected_branch: "main".to_string(),
            expected_files,
            validation_policy: VALIDATION_POLICY.to_string(),
        }
    }

    pub fn validate(&self) -> AppResult<()> {
        if self.tool != TOOL_NAME
            || self.schema_version != SCHEMA_VERSION
            || self.exercise.trim().is_empty()
            || self.created_at.trim().is_empty()
            || self.expected_branch != "main"
            || self.expected_files.is_empty()
            || self.validation_policy != VALIDATION_POLICY
        {
            return Err(invalid_state("State fields do not match the v0.1 schema."));
        }
        Ok(())
    }
}

pub fn read_state(path: &Path) -> AppResult<BranchDojoState> {
    let state_path = path.join(STATE_FILE);
    if !state_path.exists() {
        return Err(AppError::new(
            "BD004",
            ".branchdojo.json is missing. This is not a BranchDojo workspace.",
            "`check`, `hint`, and `reset` require generated workspace metadata.",
            "Create an exercise with `branchdojo new <exercise> --path <path>`.",
        ));
    }

    let content = fs::read_to_string(&state_path)
        .map_err(|error| AppError::io("Could not read .branchdojo.json.", error))?;
    let state = parse_state(&content)?;
    state.validate()?;
    Ok(state)
}

pub fn write_state(path: &Path, state: &BranchDojoState) -> AppResult<()> {
    state.validate()?;
    fs::write(path.join(STATE_FILE), to_json(state))
        .map_err(|error| AppError::io("Could not write .branchdojo.json.", error))
}

pub fn to_json(state: &BranchDojoState) -> String {
    let files = state
        .expected_files
        .iter()
        .map(|file| format!("\"{}\"", escape_json(file)))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{{\n  \"tool\": \"{}\",\n  \"schema_version\": \"{}\",\n  \"exercise\": \"{}\",\n  \"created_at\": \"{}\",\n  \"expected_branch\": \"{}\",\n  \"expected_files\": [{}],\n  \"validation_policy\": \"{}\"\n}}\n",
        escape_json(&state.tool),
        escape_json(&state.schema_version),
        escape_json(&state.exercise),
        escape_json(&state.created_at),
        escape_json(&state.expected_branch),
        files,
        escape_json(&state.validation_policy)
    )
}

fn parse_state(content: &str) -> AppResult<BranchDojoState> {
    let state = BranchDojoState {
        tool: extract_string(content, "tool")?,
        schema_version: extract_string(content, "schema_version")?,
        exercise: extract_string(content, "exercise")?,
        created_at: extract_string(content, "created_at")?,
        expected_branch: extract_string(content, "expected_branch")?,
        expected_files: extract_string_array(content, "expected_files")?,
        validation_policy: extract_string(content, "validation_policy")?,
    };
    Ok(state)
}

fn extract_string(content: &str, key: &str) -> AppResult<String> {
    let marker = format!("\"{key}\"");
    let start = content
        .find(&marker)
        .ok_or_else(|| invalid_state(format!("Missing `{key}`.")))?;
    let after_key = &content[start + marker.len()..];
    let colon = after_key
        .find(':')
        .ok_or_else(|| invalid_state(format!("Missing value for `{key}`.")))?;
    let after_colon = after_key[colon + 1..].trim_start();
    if !after_colon.starts_with('"') {
        return Err(invalid_state(format!("`{key}` must be a string.")));
    }
    read_json_string(after_colon).ok_or_else(|| invalid_state(format!("Invalid `{key}` string.")))
}

fn extract_string_array(content: &str, key: &str) -> AppResult<Vec<String>> {
    let marker = format!("\"{key}\"");
    let start = content
        .find(&marker)
        .ok_or_else(|| invalid_state(format!("Missing `{key}`.")))?;
    let after_key = &content[start + marker.len()..];
    let colon = after_key
        .find(':')
        .ok_or_else(|| invalid_state(format!("Missing value for `{key}`.")))?;
    let after_colon = after_key[colon + 1..].trim_start();
    if !after_colon.starts_with('[') {
        return Err(invalid_state(format!("`{key}` must be an array.")));
    }
    let end = after_colon
        .find(']')
        .ok_or_else(|| invalid_state(format!("Invalid `{key}` array.")))?;
    let inner = &after_colon[1..end];
    let mut values = Vec::new();
    for raw in inner.split(',') {
        let value = raw.trim();
        if value.is_empty() {
            continue;
        }
        if !value.starts_with('"') {
            return Err(invalid_state(format!("`{key}` entries must be strings.")));
        }
        values.push(
            read_json_string(value)
                .ok_or_else(|| invalid_state(format!("Invalid `{key}` entry.")))?,
        );
    }
    Ok(values)
}

fn read_json_string(input: &str) -> Option<String> {
    let mut value = String::new();
    let mut escaped = false;
    for character in input[1..].chars() {
        if escaped {
            value.push(match character {
                '"' => '"',
                '\\' => '\\',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                other => other,
            });
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == '"' {
            return Some(value);
        } else {
            value.push(character);
        }
    }
    None
}

pub fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn invalid_state(detail: impl Into<String>) -> AppError {
    AppError::new(
        "BD005",
        ".branchdojo.json is invalid.",
        detail,
        "Reset only if the workspace is valid, or create a new exercise.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_round_trip_works() {
        let state = BranchDojoState::new("conflict-basic", vec!["app.txt".to_string()]);
        let parsed = parse_state(&to_json(&state)).unwrap();
        assert_eq!(parsed, state);
    }

    #[test]
    fn invalid_tool_is_rejected() {
        let mut state = BranchDojoState::new("conflict-basic", vec!["app.txt".to_string()]);
        state.tool = "other".to_string();
        assert!(state.validate().is_err());
    }
}
