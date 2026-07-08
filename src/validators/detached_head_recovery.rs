use std::path::Path;

use crate::error::AppResult;
use crate::exercises::detached_head_recovery::{
    DETACHED_COMMIT_MESSAGE, RECOVERED_CONTENT, RECOVERED_FILE, RECOVERY_BRANCH,
};
use crate::git;
use crate::result::{CheckResult, CheckStatus, Severity, ValidationResult};
use crate::state::BranchDojoState;
use crate::validators::common;

pub fn validate(path: &Path, state: &BranchDojoState) -> AppResult<ValidationResult> {
    let mut checks = vec![
        common::metadata_exists(path),
        common::git_directory_exists(path),
        current_branch_recovery(path),
        common::working_tree_clean(path),
        common::no_active_git_operation(path),
        common::branch_exists(path, "main"),
        common::branch_exists(path, RECOVERY_BRANCH),
        common::branch_file_contains(
            path,
            RECOVERY_BRANCH,
            RECOVERED_FILE,
            "recovered_work_exists",
            RECOVERED_CONTENT,
            "Recovered work exists on recovery branch",
        ),
        no_longer_detached(path),
    ];

    let required_checks_pass = checks
        .iter()
        .filter(|check| check.severity == Severity::Required)
        .all(|check| check.status == CheckStatus::Passed);
    let detached_subject_reachable =
        branch_log_contains(path, RECOVERY_BRANCH, DETACHED_COMMIT_MESSAGE);
    checks.push(CheckResult::warning(
        "detached_commit_preserved",
        "Detached commit preservation check",
        required_checks_pass && !detached_subject_reachable,
    ));

    Ok(ValidationResult::new(
        &state.exercise,
        checks,
        vec![
            "Create `recovery/detached-work` from the detached commit, make sure recovered-note.txt is present, then run branchdojo check --path . again.".to_string(),
            "If the result is a warning, inspect the graph and try preserving the original detached commit directly.".to_string(),
        ],
    ))
}

fn current_branch_recovery(path: &Path) -> CheckResult {
    let passed = git::current_branch(path)
        .map(|branch| branch == RECOVERY_BRANCH)
        .unwrap_or(false);
    CheckResult::required(
        "current_branch_recovery_detached_work",
        "Current branch is recovery/detached-work",
        passed,
    )
}

fn no_longer_detached(path: &Path) -> CheckResult {
    let passed = git::current_branch(path)
        .map(|branch| !branch.trim().is_empty())
        .unwrap_or(false);
    CheckResult::required(
        "not_detached_head",
        "Repository is not in detached HEAD",
        passed,
    )
}

fn branch_log_contains(path: &Path, branch: &str, message: &str) -> bool {
    git::run_git(path, ["log", branch, "--format=%s"])
        .map(|log| log.lines().any(|line| line == message))
        .unwrap_or(false)
}
