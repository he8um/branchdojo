use std::fs;
use std::path::Path;

use crate::git;
use crate::result::CheckResult;
use crate::state::STATE_FILE;

pub fn normalize(value: &str) -> String {
    value.replace("\r\n", "\n").replace('\r', "\n")
}

pub fn metadata_exists(path: &Path) -> CheckResult {
    CheckResult::required(
        "metadata_exists",
        "Metadata exists",
        path.join(STATE_FILE).exists(),
    )
}

pub fn git_directory_exists(path: &Path) -> CheckResult {
    CheckResult::required(
        "git_directory_exists",
        "Git directory exists",
        path.join(".git").exists(),
    )
}

pub fn current_branch_main(path: &Path) -> CheckResult {
    let passed = git::current_branch(path)
        .map(|branch| branch == "main")
        .unwrap_or(false);
    CheckResult::required("current_branch_main", "Current branch is main", passed)
}

pub fn working_tree_clean(path: &Path) -> CheckResult {
    let passed = git::status_porcelain(path)
        .map(|status| status.trim().is_empty())
        .unwrap_or(false);
    CheckResult::required("working_tree_clean", "Working tree is clean", passed)
}

pub fn no_active_git_operation(path: &Path) -> CheckResult {
    CheckResult::required(
        "no_active_git_operation",
        "No merge/rebase/cherry-pick/revert state is active",
        !git::active_operation(path),
    )
}

pub fn file_exists(path: &Path, file: &str) -> CheckResult {
    CheckResult::required(
        format!("file_exists:{file}"),
        format!("{file} exists"),
        path.join(file).exists(),
    )
}

pub fn file_contains(
    path: &Path,
    file: &str,
    id: &str,
    text: &str,
    label: &str,
) -> CheckResult {
    let passed = fs::read_to_string(path.join(file))
        .map(|content| normalize(&content).contains(text))
        .unwrap_or(false);
    CheckResult::required(format!("file_contains:{id}"), label, passed)
}

pub fn file_not_contains(
    path: &Path,
    file: &str,
    id: &str,
    text: &str,
    label: &str,
) -> CheckResult {
    let passed = fs::read_to_string(path.join(file))
        .map(|content| !normalize(&content).contains(text))
        .unwrap_or(false);
    CheckResult::required(format!("file_not_contains:{id}"), label, passed)
}

pub fn branch_exists(path: &Path, branch: &str) -> CheckResult {
    let passed = git::branch_exists(path, branch).unwrap_or(false);
    CheckResult::required(
        format!("branch_exists:{branch}"),
        format!("Branch `{branch}` exists"),
        passed,
    )
}

pub fn branch_file_contains(
    path: &Path,
    branch: &str,
    file: &str,
    id: &str,
    text: &str,
    label: &str,
) -> CheckResult {
    let passed = git::file_content_at_branch(path, branch, file)
        .map(|content| {
            content
                .map(|value| normalize(&value).contains(text))
                .unwrap_or(false)
        })
        .unwrap_or(false);
    CheckResult::required(id, label, passed)
}

pub fn branch_file_not_contains(
    path: &Path,
    branch: &str,
    file: &str,
    id: &str,
    text: &str,
    label: &str,
) -> CheckResult {
    let passed = git::file_content_at_branch(path, branch, file)
        .map(|content| {
            content
                .map(|value| !normalize(&value).contains(text))
                .unwrap_or(true)
        })
        .unwrap_or(false);
    CheckResult::required(id, label, passed)
}

pub fn no_conflict_markers(path: &Path, file: &str) -> CheckResult {
    let passed = fs::read_to_string(path.join(file))
        .map(|content| {
            let normalized = normalize(&content);
            !normalized.contains("<<<<<<<")
                && !normalized.contains("=======")
                && !normalized.contains(">>>>>>>")
        })
        .unwrap_or(false);
    CheckResult::required("conflict_markers_removed", "Conflict markers removed", passed)
}
