use std::path::Path;

use crate::error::AppResult;
use crate::exercises::cherry_pick_basic::{
    APP_FILE, BUGFIX_COMMIT_MESSAGE, BUGFIX_CONTENT, LEGACY_CONTENT, LEGACY_FILE, RELEASE_BRANCH,
    SUPPORT_BRANCH,
};
use crate::git;
use crate::result::{CheckResult, CheckStatus, Severity, ValidationResult};
use crate::state::BranchDojoState;
use crate::validators::common;

pub fn validate(path: &Path, state: &BranchDojoState) -> AppResult<ValidationResult> {
    let mut checks = vec![
        common::metadata_exists(path),
        common::git_directory_exists(path),
        current_branch_release(path),
        common::working_tree_clean(path),
        common::no_active_git_operation(path),
        common::branch_exists(path, SUPPORT_BRANCH),
        common::branch_exists(path, RELEASE_BRANCH),
        common::branch_file_contains(
            path,
            RELEASE_BRANCH,
            APP_FILE,
            "bugfix_exists_on_release",
            BUGFIX_CONTENT,
            "Release branch has checkout bugfix",
        ),
        common::branch_file_not_contains(
            path,
            RELEASE_BRANCH,
            LEGACY_FILE,
            "legacy_content_absent_on_release",
            LEGACY_CONTENT,
            "Release branch excludes legacy-only content",
        ),
        source_commit_still_exists(path),
        common::branch_file_contains(
            path,
            SUPPORT_BRANCH,
            APP_FILE,
            "bugfix_remains_on_support",
            BUGFIX_CONTENT,
            "Support branch still has checkout bugfix",
        ),
    ];

    let required_checks_pass = checks
        .iter()
        .filter(|check| check.severity == Severity::Required)
        .all(|check| check.status == CheckStatus::Passed);
    let cherry_pick_signal = branch_log_contains(path, RELEASE_BRANCH, BUGFIX_COMMIT_MESSAGE);
    let merge_signal = branch_has_merge_commit(path, RELEASE_BRANCH);
    checks.push(CheckResult::warning(
        "cherry_pick_style_detected",
        "Cherry-pick-style workflow check",
        required_checks_pass && !cherry_pick_signal,
    ));
    checks.push(CheckResult::warning(
        "broad_merge_shape_detected",
        "Broad merge shape check",
        required_checks_pass && merge_signal,
    ));

    Ok(ValidationResult::new(
        &state.exercise,
        checks,
        vec![
            "Apply only the `Fix empty checkout cart` commit to release/current, keep legacy-only content out, then run branchdojo check --path . again.".to_string(),
            "If the result is a warning, inspect the branch graph and try solving this again with a direct cherry-pick.".to_string(),
        ],
    ))
}

fn current_branch_release(path: &Path) -> CheckResult {
    let passed = git::current_branch(path)
        .map(|branch| branch == RELEASE_BRANCH)
        .unwrap_or(false);
    CheckResult::required(
        "current_branch_release_current",
        "Current branch is release/current",
        passed,
    )
}

fn source_commit_still_exists(path: &Path) -> CheckResult {
    CheckResult::required(
        "source_commit_still_exists",
        "Source bugfix commit still exists",
        branch_log_contains(path, SUPPORT_BRANCH, BUGFIX_COMMIT_MESSAGE),
    )
}

fn branch_log_contains(path: &Path, branch: &str, message: &str) -> bool {
    git::run_git(path, ["log", branch, "--format=%s"])
        .map(|log| log.lines().any(|line| line == message))
        .unwrap_or(false)
}

fn branch_has_merge_commit(path: &Path, branch: &str) -> bool {
    git::run_git(path, ["log", branch, "--merges", "--format=%H"])
        .map(|log| !log.trim().is_empty())
        .unwrap_or(false)
}
