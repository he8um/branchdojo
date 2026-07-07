use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

pub const STATE_FILE: &str = ".branchdojo.json";
pub const TOOL_NAME: &str = "branchdojo";
pub const SCHEMA_VERSION: &str = "0.1.0";
pub const VALIDATION_POLICY: &str = "final-state";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
            created_at: created_at_now(),
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
    let state: BranchDojoState = serde_json::from_str(&content)
        .map_err(|error| invalid_state(format!("Could not parse state JSON: {error}")))?;
    state.validate()?;
    Ok(state)
}

pub fn write_state(path: &Path, state: &BranchDojoState) -> AppResult<()> {
    state.validate()?;
    let content = serde_json::to_string_pretty(state)
        .map_err(|error| invalid_state(format!("Could not serialize state JSON: {error}")))?;
    fs::write(path.join(STATE_FILE), format!("{content}\n"))
        .map_err(|error| AppError::io("Could not write .branchdojo.json.", error))
}

fn invalid_state(detail: impl Into<String>) -> AppError {
    AppError::new(
        "BD005",
        ".branchdojo.json is invalid.",
        detail,
        "Reset only if the workspace is valid, or create a new exercise.",
    )
}

fn created_at_now() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_round_trip_works() {
        let state = BranchDojoState::new("conflict-basic", vec!["app.txt".to_string()]);
        let json = serde_json::to_string(&state).unwrap();
        let parsed: BranchDojoState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, state);
    }

    #[test]
    fn invalid_tool_is_rejected() {
        let mut state = BranchDojoState::new("conflict-basic", vec!["app.txt".to_string()]);
        state.tool = "other".to_string();
        assert!(state.validate().is_err());
    }
}
