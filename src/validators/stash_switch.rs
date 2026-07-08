use std::path::Path;

use crate::error::AppResult;
use crate::exercises::stash_switch::{APP_FILE, FEATURE_BRANCH, PRESERVED_WORK, SETTINGS_COPY};
use crate::git;
use crate::result::{CheckResult, CheckStatus, Severity, ValidationResult};
use crate::state::BranchDojoState;
use crate::validators::common;

pub fn validate(path: &Path, state: &BranchDojoState) -> AppResult<ValidationResult> {
    let mut checks = vec![
        common::metadata_exists(path),
        common::git_directory_exists(path),
        common::current_branch_main(path),
        common::working_tree_clean(path),
        common::no_active_git_operation(path),
        common::file_exists(path, APP_FILE),
        common::branch_exists(path, FEATURE_BRANCH),
        common::branch_file_contains(
            path,
            FEATURE_BRANCH,
            APP_FILE,
            "feature_settings_copy_exists",
            SETTINGS_COPY,
            "Feature branch has expected settings copy",
        ),
        common::branch_file_contains(
            path,
            "main",
            APP_FILE,
            "preserved_work_exists",
            PRESERVED_WORK,
            "Preserved local work exists on main",
        ),
    ];

    let required_checks_pass = checks
        .iter()
        .filter(|check| check.severity == Severity::Required)
        .all(|check| check.status == CheckStatus::Passed);
    checks.push(CheckResult::warning(
        "wip_commit_detected",
        "WIP commit check",
        required_checks_pass && wip_commit_detected(path),
    ));
    checks.push(CheckResult::warning(
        "feature_copy_on_main",
        "Feature copy on main check",
        required_checks_pass && feature_copy_on_main(path),
    ));

    Ok(ValidationResult::new(
        &state.exercise,
        checks,
        vec![
            "Preserve the local work, update feature/settings-copy with the settings copy, return to main, commit the preserved work, then run branchdojo check --path . again.".to_string(),
            "If the result is a warning, try solving this again by stashing before switching branches.".to_string(),
        ],
    ))
}

fn wip_commit_detected(path: &Path) -> bool {
    git::run_git(path, ["log", "--format=%s", "main"])
        .map(|log| {
            log.lines().any(|line| {
                let lower = line.to_ascii_lowercase();
                lower.contains("wip") || lower.contains("work in progress")
            })
        })
        .unwrap_or(false)
}

fn feature_copy_on_main(path: &Path) -> bool {
    git::file_content_at_branch(path, "main", APP_FILE)
        .map(|content| {
            content.is_some_and(|value| common::normalize(&value).contains(SETTINGS_COPY))
        })
        .unwrap_or(false)
}
