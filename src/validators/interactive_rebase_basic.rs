use std::path::Path;

use crate::error::AppResult;
use crate::exercises::interactive_rebase_basic::{
    DEBUG_CONTENT, DEBUG_FILE, FEATURE_BRANCH, PROFILE_FILE, PROFILE_MANAGE, PROFILE_TITLE,
    PROFILE_WELCOME, TYPO_COMMIT_MESSAGE, WIP_COMMIT_MESSAGE,
};
use crate::git;
use crate::result::{CheckResult, CheckStatus, Severity, ValidationResult};
use crate::state::BranchDojoState;
use crate::validators::common;

pub fn validate(path: &Path, state: &BranchDojoState) -> AppResult<ValidationResult> {
    let mut checks = vec![
        common::metadata_exists(path),
        common::git_directory_exists(path),
        current_branch_feature_profile_copy(path),
        common::working_tree_clean(path),
        common::no_active_git_operation(path),
        common::branch_exists(path, "main"),
        common::branch_exists(path, FEATURE_BRANCH),
        common::file_exists(path, PROFILE_FILE),
        common::file_contains(
            path,
            PROFILE_FILE,
            "profile_title",
            PROFILE_TITLE,
            "Profile title exists",
        ),
        common::file_contains(
            path,
            PROFILE_FILE,
            "profile_welcome",
            PROFILE_WELCOME,
            "Profile welcome copy exists",
        ),
        common::file_contains(
            path,
            PROFILE_FILE,
            "profile_manage",
            PROFILE_MANAGE,
            "Profile account-management copy exists",
        ),
        debug_file_absent(path),
        debug_content_absent(path),
    ];

    let required_checks_pass = checks
        .iter()
        .filter(|check| check.severity == Severity::Required)
        .all(|check| check.status == CheckStatus::Passed);
    checks.push(CheckResult::warning(
        "wip_debug_commit_reachable",
        "WIP/debug commit cleanup check",
        required_checks_pass && branch_log_has_wip_or_debug(path),
    ));
    checks.push(CheckResult::warning(
        "reviewable_history_shape",
        "Reviewable history shape check",
        required_checks_pass && !reviewable_history_shape(path),
    ));

    Ok(ValidationResult::new(
        &state.exercise,
        checks,
        vec![
            "Clean `feature/profile-copy` so profile.txt keeps the final copy, debug.txt is gone, and WIP/debug history is not reachable, then run branchdojo check --path . again.".to_string(),
            "If the result is a warning, inspect the graph and try solving again with a cleaner history rewrite.".to_string(),
        ],
    ))
}

fn current_branch_feature_profile_copy(path: &Path) -> CheckResult {
    let passed = git::current_branch(path)
        .map(|branch| branch == FEATURE_BRANCH)
        .unwrap_or(false);
    CheckResult::required(
        "current_branch_feature_profile_copy",
        "Current branch is feature/profile-copy",
        passed,
    )
}

fn debug_file_absent(path: &Path) -> CheckResult {
    let passed = git::file_content_at_branch(path, FEATURE_BRANCH, DEBUG_FILE)
        .map(|content| content.is_none())
        .unwrap_or(false);
    CheckResult::required("debug_file_absent", "Debug file is absent", passed)
}

fn debug_content_absent(path: &Path) -> CheckResult {
    common::branch_file_not_contains(
        path,
        FEATURE_BRANCH,
        DEBUG_FILE,
        "debug_content_absent",
        DEBUG_CONTENT,
        "Debug content is absent",
    )
}

fn branch_log_has_wip_or_debug(path: &Path) -> bool {
    git::run_git(path, ["log", FEATURE_BRANCH, "--format=%s"])
        .map(|log| {
            log.lines().any(|line| {
                let lower = line.to_ascii_lowercase();
                lower.contains("wip") || lower.contains("debug")
            })
        })
        .unwrap_or(false)
}

fn reviewable_history_shape(path: &Path) -> bool {
    let log =
        git::run_git(path, ["log", FEATURE_BRANCH, "^main", "--format=%s"]).unwrap_or_default();
    let subjects: Vec<_> = log.lines().collect();
    !subjects.is_empty()
        && subjects.len() <= 2
        && !subjects.iter().any(|subject| {
            let lower = subject.to_ascii_lowercase();
            lower.contains("wip")
                || lower.contains("debug")
                || lower.contains("fixup")
                || lower.contains("fix profile copy typo")
                || lower == TYPO_COMMIT_MESSAGE.to_ascii_lowercase()
                || lower.starts_with("fix ")
        })
        && !branch_log_has_wip_or_debug(path)
        && !subjects.contains(&WIP_COMMIT_MESSAGE)
}
