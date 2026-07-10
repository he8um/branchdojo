use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::result::ValidationResult;
use crate::state::BranchDojoState;

mod cherry_pick_basic;
mod common;
mod conflict_basic;
mod detached_head_recovery;
mod interactive_rebase_basic;
mod revert_mistake;
mod stash_switch;
mod tag_release_fix;
mod wrong_branch_commit;

pub fn validate(path: &Path, state: &BranchDojoState) -> AppResult<ValidationResult> {
    match state.exercise.as_str() {
        "cherry-pick-basic" => cherry_pick_basic::validate(path, state),
        "conflict-basic" => conflict_basic::validate(path, state),
        "detached-head-recovery" => detached_head_recovery::validate(path, state),
        "interactive-rebase-basic" => interactive_rebase_basic::validate(path, state),
        "revert-mistake" => revert_mistake::validate(path, state),
        "stash-switch" => stash_switch::validate(path, state),
        "tag-release-fix" => tag_release_fix::validate(path, state),
        "wrong-branch-commit" => wrong_branch_commit::validate(path, state),
        other => Err(AppError::new(
            "BD006",
            format!("Unsupported exercise: {other}."),
            "The exercise ID is unknown.",
            "Run `branchdojo list`.",
        )),
    }
}
