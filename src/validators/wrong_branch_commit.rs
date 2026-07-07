use std::path::Path;

use crate::error::AppResult;
use crate::exercises::wrong_branch_commit::{ACCIDENTAL_CONTENT, ACCIDENTAL_FILE, FEATURE_BRANCH};
use crate::git;
use crate::result::{CheckResult, ValidationResult};
use crate::state::BranchDojoState;
use crate::validators::common;

pub fn validate(path: &Path, state: &BranchDojoState) -> AppResult<ValidationResult> {
    let mut checks = vec![
        common::metadata_exists(path),
        common::git_directory_exists(path),
        common::current_branch_main(path),
        common::working_tree_clean(path),
        common::no_active_git_operation(path),
        common::branch_exists(path, "main"),
        common::branch_exists(path, FEATURE_BRANCH),
        common::branch_file_not_contains(
            path,
            "main",
            ACCIDENTAL_FILE,
            "main_excludes_accidental_work",
            ACCIDENTAL_CONTENT,
            "main excludes accidental work",
        ),
        common::branch_file_contains(
            path,
            FEATURE_BRANCH,
            ACCIDENTAL_FILE,
            "feature_includes_accidental_work",
            ACCIDENTAL_CONTENT,
            "feature includes accidental work",
        ),
    ];

    let original_bad_commit_on_main =
        git::log_contains_message(path, "Add profile page draft").unwrap_or(false);
    let feature_has_branch_history = git::run_git(
        path,
        ["log", FEATURE_BRANCH, "--format=%s", "--", ACCIDENTAL_FILE],
    )
    .map(|log| log.contains("Add profile page draft"))
    .unwrap_or(false);
    checks.push(CheckResult::warning(
        "history_shape_expected",
        "Final branch content is valid, but history shape is unusual",
        original_bad_commit_on_main || !feature_has_branch_history,
    ));

    Ok(ValidationResult::new(
        &state.exercise,
        checks,
        vec![
            "Move profile.md to feature/profile-page, restore main so it excludes that file, then run branchdojo check --path . again.".to_string(),
            "If the result is a warning, inspect the branch graph and try a cleaner move workflow.".to_string(),
        ],
    ))
}
