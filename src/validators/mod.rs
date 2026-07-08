use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::result::ValidationResult;
use crate::state::BranchDojoState;

mod common;
mod conflict_basic;
mod revert_mistake;
mod stash_switch;
mod wrong_branch_commit;

pub fn validate(path: &Path, state: &BranchDojoState) -> AppResult<ValidationResult> {
    match state.exercise.as_str() {
        "conflict-basic" => conflict_basic::validate(path, state),
        "revert-mistake" => revert_mistake::validate(path, state),
        "stash-switch" => stash_switch::validate(path, state),
        "wrong-branch-commit" => wrong_branch_commit::validate(path, state),
        other => Err(AppError::new(
            "BD006",
            format!("Unsupported exercise: {other}."),
            "The exercise ID is unknown.",
            "Run `branchdojo list`.",
        )),
    }
}
