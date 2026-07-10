use std::path::Path;

use crate::error::AppResult;
use crate::exercises::tag_release_fix::{
    APP_FILE, BAD_BLOCKER_CONTENT, FIXED_BLOCKER_CONTENT, FIX_COMMIT_MESSAGE,
    PREFERRED_TAG_MESSAGE, RELEASE_READY_CONTENT, RELEASE_TAG, VERSION_CONTENT, VERSION_FILE,
};
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
        common::branch_exists(path, "main"),
        tag_exists(path),
        common::file_exists(path, VERSION_FILE),
        common::file_contains(
            path,
            VERSION_FILE,
            "version_1_0_0",
            VERSION_CONTENT,
            "VERSION contains 1.0.0",
        ),
        common::file_contains(
            path,
            APP_FILE,
            "main_release_blocker_fixed",
            FIXED_BLOCKER_CONTENT,
            "Main has release blocker fixed",
        ),
        common::file_contains(
            path,
            APP_FILE,
            "main_release_ready",
            RELEASE_READY_CONTENT,
            "Main is release ready",
        ),
        tag_file_contains(
            path,
            APP_FILE,
            "tag_release_blocker_fixed",
            FIXED_BLOCKER_CONTENT,
            "Tag points to release blocker fix",
        ),
        tag_file_contains(
            path,
            APP_FILE,
            "tag_release_ready",
            RELEASE_READY_CONTENT,
            "Tag points to release-ready content",
        ),
        tag_file_not_contains(
            path,
            APP_FILE,
            "tag_bad_blocker_absent",
            BAD_BLOCKER_CONTENT,
            "Tagged commit excludes old blocker",
        ),
    ];

    let required_checks_pass = checks
        .iter()
        .filter(|check| check.severity == Severity::Required)
        .all(|check| check.status == CheckStatus::Passed);
    checks.push(CheckResult::warning(
        "release_tag_is_annotated",
        "Release tag annotation check",
        required_checks_pass && !tag_is_annotated(path),
    ));
    checks.push(CheckResult::warning(
        "release_tag_message_is_ideal",
        "Release tag message check",
        required_checks_pass && !tag_message_is_ideal(path),
    ));
    checks.push(CheckResult::warning(
        "release_tag_points_to_fix_commit",
        "Release tag target shape check",
        required_checks_pass && !tag_points_to_fix_commit(path),
    ));

    Ok(ValidationResult::new(
        &state.exercise,
        checks,
        vec![
            "Move or recreate `v1.0.0` so it points to the commit with release_blocker=false and release_ready=true, then run branchdojo check --path . again.".to_string(),
            "If the result is a warning, inspect the tag object and prefer an annotated release tag with a clear message.".to_string(),
        ],
    ))
}

fn tag_exists(path: &Path) -> CheckResult {
    CheckResult::required(
        "tag_exists:v1.0.0",
        "Tag `v1.0.0` exists",
        git::run_git(path, ["rev-parse", "--verify", "--quiet", RELEASE_TAG]).is_ok(),
    )
}

fn tag_file_contains(path: &Path, file: &str, id: &str, text: &str, label: &str) -> CheckResult {
    let passed = tag_file_content(path, file)
        .map(|content| common::normalize(&content).contains(text))
        .unwrap_or(false);
    CheckResult::required(id, label, passed)
}

fn tag_file_not_contains(
    path: &Path,
    file: &str,
    id: &str,
    text: &str,
    label: &str,
) -> CheckResult {
    let passed = tag_file_content(path, file)
        .map(|content| !common::normalize(&content).contains(text))
        .unwrap_or(false);
    CheckResult::required(id, label, passed)
}

fn tag_file_content(path: &Path, file: &str) -> Option<String> {
    let spec = format!("{RELEASE_TAG}:{file}");
    git::run_git(path, ["show", &spec]).ok()
}

fn tag_is_annotated(path: &Path) -> bool {
    git::run_git(path, ["cat-file", "-t", RELEASE_TAG])
        .map(|kind| kind.trim() == "tag")
        .unwrap_or(false)
}

fn tag_message_is_ideal(path: &Path) -> bool {
    git::run_git(path, ["tag", "-l", RELEASE_TAG, "--format=%(contents)"])
        .map(|message| message.contains(PREFERRED_TAG_MESSAGE))
        .unwrap_or(false)
}

fn tag_points_to_fix_commit(path: &Path) -> bool {
    let tag_commit = git::run_git(path, ["rev-parse", &format!("{RELEASE_TAG}^{{}}")])
        .map(|value| value.trim().to_string())
        .unwrap_or_default();
    let fix_commit = git::run_git(
        path,
        [
            "log",
            "main",
            "--format=%H",
            "--grep",
            &format!("^{FIX_COMMIT_MESSAGE}$"),
            "-n",
            "1",
        ],
    )
    .map(|value| value.trim().to_string())
    .unwrap_or_default();
    !tag_commit.is_empty() && tag_commit == fix_commit
}
