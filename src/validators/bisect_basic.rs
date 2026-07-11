use std::fs;
use std::path::Path;

use crate::error::AppResult;
use crate::exercises::bisect_basic::{
    APP_FILE, BAD_MARKER, CHECK_FILE, CULPRIT_COMMIT_MESSAGE, DIAGNOSIS_FILE, EXPECTED_MARKER,
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
        no_active_bisect_state(path),
        common::branch_exists(path, "main"),
        common::file_exists(path, APP_FILE),
        common::file_exists(path, CHECK_FILE),
        common::file_contains(
            path,
            APP_FILE,
            "current_discount_regression",
            BAD_MARKER,
            "Current app has discount regression",
        ),
        common::file_contains(
            path,
            CHECK_FILE,
            "expected_discount_total",
            EXPECTED_MARKER,
            "Check fixture has expected discount total",
        ),
        common::file_exists(path, DIAGNOSIS_FILE),
        diagnosis_identifies_culprit(path),
        culprit_commit_reachable(path),
    ];

    let required_checks_pass = checks
        .iter()
        .filter(|check| check.severity == Severity::Required)
        .all(|check| check.status == CheckStatus::Passed);
    checks.push(CheckResult::warning(
        "diagnosis_lacks_subject",
        "Diagnosis includes culprit subject",
        required_checks_pass && !diagnosis_contains_subject(path),
    ));
    checks.push(CheckResult::warning(
        "diagnosis_lacks_explanation",
        "Diagnosis includes supporting detail",
        required_checks_pass && !diagnosis_has_supporting_detail(path),
    ));
    checks.push(CheckResult::warning(
        "diagnostic_branch_exists",
        "Diagnostic branch cleanup check",
        required_checks_pass && diagnostic_branch_exists(path),
    ));

    Ok(ValidationResult::new(
        &state.exercise,
        checks,
        vec![
            "Create diagnosis.md and identify the commit that introduced the discount regression, then run branchdojo check --path . again.".to_string(),
            "If a bisect session is active, finish or reset it before final validation.".to_string(),
        ],
    ))
}

fn no_active_bisect_state(path: &Path) -> CheckResult {
    CheckResult::required(
        "no_active_bisect_state",
        "No active bisect state is present",
        !bisect_state_exists(path),
    )
}

fn diagnosis_identifies_culprit(path: &Path) -> CheckResult {
    CheckResult::required(
        "diagnosis_identifies_culprit",
        "Diagnosis identifies culprit commit",
        diagnosis_is_correct(path),
    )
}

fn culprit_commit_reachable(path: &Path) -> CheckResult {
    CheckResult::required(
        "culprit_commit_reachable",
        "Culprit commit is reachable from main",
        culprit_hash(path)
            .map(|hash| {
                git::run_git(path, ["merge-base", "--is-ancestor", hash.as_str(), "main"]).is_ok()
            })
            .unwrap_or(false),
    )
}

fn diagnosis_is_correct(path: &Path) -> bool {
    let Some(diagnosis) = diagnosis(path) else {
        return false;
    };
    let normalized = common::normalize(&diagnosis);
    normalized.contains(CULPRIT_COMMIT_MESSAGE)
        || normalized.contains(BAD_MARKER)
        || diagnosis_contains_correct_hash(path, &normalized)
}

fn diagnosis_contains_subject(path: &Path) -> bool {
    diagnosis(path)
        .map(|content| common::normalize(&content).contains(CULPRIT_COMMIT_MESSAGE))
        .unwrap_or(false)
}

fn diagnosis_has_supporting_detail(path: &Path) -> bool {
    diagnosis(path)
        .map(|content| {
            let normalized = common::normalize(&content);
            normalized.contains(BAD_MARKER)
                || normalized.contains("discount")
                || normalized.contains("regression")
        })
        .unwrap_or(false)
}

fn diagnosis_contains_correct_hash(path: &Path, diagnosis: &str) -> bool {
    let Some(culprit) = culprit_hash(path) else {
        return false;
    };
    hash_candidates(diagnosis).any(|candidate| {
        resolve_commit(path, candidate)
            .map(|resolved| resolved == culprit)
            .unwrap_or(false)
    })
}

fn hash_candidates(diagnosis: &str) -> impl Iterator<Item = &str> {
    diagnosis
        .split(|character: char| !character.is_ascii_hexdigit())
        .filter(|candidate| (7..=40).contains(&candidate.len()))
}

fn resolve_commit(path: &Path, candidate: &str) -> Option<String> {
    git::run_git(
        path,
        ["rev-parse", "--verify", &format!("{candidate}^{{commit}}")],
    )
    .ok()
    .map(|value| value.trim().to_string())
    .filter(|value| !value.is_empty())
}

fn culprit_hash(path: &Path) -> Option<String> {
    git::run_git(
        path,
        [
            "log",
            "main",
            "--format=%H",
            "--grep",
            &format!("^{CULPRIT_COMMIT_MESSAGE}$"),
            "-n",
            "1",
        ],
    )
    .ok()
    .map(|value| value.trim().to_string())
    .filter(|value| !value.is_empty())
}

fn diagnosis(path: &Path) -> Option<String> {
    fs::read_to_string(path.join(DIAGNOSIS_FILE)).ok()
}

fn bisect_state_exists(path: &Path) -> bool {
    let git_dir = path.join(".git");
    ["BISECT_LOG", "BISECT_START", "BISECT_NAMES", "BISECT_TERMS"]
        .iter()
        .any(|file| git_dir.join(file).exists())
}

fn diagnostic_branch_exists(path: &Path) -> bool {
    git::branch_exists(path, "debug/bisect-result").unwrap_or(false)
}
