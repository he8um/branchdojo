use std::path::Path;

use crate::error::AppResult;
use crate::exercises::conflict_basic::{EXPECTED_CTA, EXPECTED_HEADLINE, FEATURE_BRANCH};
use crate::git;
use crate::result::{CheckResult, ValidationResult};
use crate::state::BranchDojoState;
use crate::validators::common;

const APP_FILE: &str = "app.txt";

pub fn validate(path: &Path, state: &BranchDojoState) -> AppResult<ValidationResult> {
    let mut checks = vec![
        common::metadata_exists(path),
        common::git_directory_exists(path),
        common::current_branch_main(path),
        common::working_tree_clean(path),
        common::no_active_git_operation(path),
        common::file_exists(path, APP_FILE),
        common::no_conflict_markers(path, APP_FILE),
        common::file_contains(
            path,
            APP_FILE,
            "headline",
            EXPECTED_HEADLINE,
            "Expected headline exists",
        ),
        common::file_contains(path, APP_FILE, "cta", EXPECTED_CTA, "Expected CTA exists"),
        common::branch_exists(path, FEATURE_BRANCH),
    ];

    let history_includes_feature =
        git::log_contains_message(path, "Update landing CTA").unwrap_or(false);
    checks.push(CheckResult::required(
        "history_includes_feature_work",
        "History includes feature work",
        history_includes_feature,
    ));

    let no_merge_commit = !git::merge_commit_exists(path).unwrap_or(false);
    checks.push(CheckResult::warning(
        "merge_commit_detected",
        "Final state is valid, but no merge commit was detected",
        no_merge_commit,
    ));

    Ok(ValidationResult::new(
        &state.exercise,
        checks,
        vec![
            "Open app.txt, remove conflict markers, preserve both required lines, then run branchdojo check --path . again.".to_string(),
            "If the result is a warning, try solving it again with a merge commit.".to_string(),
        ],
    ))
}
