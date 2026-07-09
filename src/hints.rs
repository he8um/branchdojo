use std::fs;
use std::path::Path;

use crate::error::AppResult;
use crate::exercises::cherry_pick_basic::{
    APP_FILE as CHERRY_APP_FILE, BUGFIX_CONTENT, LEGACY_CONTENT, LEGACY_FILE, RELEASE_BRANCH,
    SUPPORT_BRANCH,
};
use crate::exercises::conflict_basic::{EXPECTED_CTA, FEATURE_BRANCH as CONFLICT_FEATURE_BRANCH};
use crate::exercises::detached_head_recovery::{
    RECOVERED_CONTENT, RECOVERED_FILE, RECOVERY_BRANCH,
};
use crate::exercises::revert_mistake::{CONFIG_FILE, UNSAFE_VALUE};
use crate::exercises::stash_switch::{
    APP_FILE as STASH_APP_FILE, FEATURE_BRANCH as STASH_FEATURE_BRANCH, SETTINGS_COPY,
};
use crate::exercises::wrong_branch_commit::{
    ACCIDENTAL_CONTENT, ACCIDENTAL_FILE, FEATURE_BRANCH as WRONG_BRANCH_FEATURE,
};
use crate::git;
use crate::state::BranchDojoState;

pub struct HintContext {
    pub exercise: String,
    pub expected_branch: String,
    pub current_branch: Option<String>,
    pub is_detached: bool,
    pub working_tree_clean: bool,
    pub has_conflict_markers: bool,
    pub active_operation: Option<String>,
    pub expected_branch_exists: bool,
    pub missing_expected_files: Vec<String>,
}

pub fn hints_for(path: &Path, state: &BranchDojoState) -> AppResult<Vec<String>> {
    let context = HintContext::from_workspace(path, state)?;
    let mut hints = generic_hints(&context);
    hints.extend(exercise_hints(path, &context));
    if hints.is_empty() {
        hints.push("No obvious blocker detected from the current repository state.".to_string());
    }
    Ok(hints)
}

impl HintContext {
    fn from_workspace(path: &Path, state: &BranchDojoState) -> AppResult<Self> {
        let current_branch_raw = git::current_branch(path)?;
        let current_branch = if current_branch_raw.trim().is_empty() {
            None
        } else {
            Some(current_branch_raw)
        };
        let is_detached = current_branch.is_none();
        let status = git::status_porcelain(path)?;
        let expected_branch_exists = git::branch_exists(path, &state.expected_branch)?;
        let missing_expected_files = state
            .expected_files
            .iter()
            .filter(|file| !path.join(file).exists())
            .cloned()
            .collect();

        Ok(Self {
            exercise: state.exercise.clone(),
            expected_branch: state.expected_branch.clone(),
            current_branch,
            is_detached,
            working_tree_clean: status.trim().is_empty(),
            has_conflict_markers: has_conflict_markers(path, &state.expected_files),
            active_operation: active_operation_name(path),
            expected_branch_exists,
            missing_expected_files,
        })
    }
}

fn generic_hints(context: &HintContext) -> Vec<String> {
    let mut hints = Vec::new();

    if let Some(operation) = &context.active_operation {
        hints.push(format!(
            "A Git {operation} operation appears to be in progress. Complete or abort it before checking."
        ));
    }

    if context.has_conflict_markers {
        hints.push(
            "Conflict markers are still present. Resolve them before final validation.".to_string(),
        );
    }

    if context.is_detached && context.expected_branch != "detached HEAD" {
        hints.push(format!(
            "This repository is still in detached HEAD state. Create or switch to `{}` before checking.",
            context.expected_branch
        ));
    } else if let Some(current_branch) = &context.current_branch {
        if current_branch != &context.expected_branch {
            hints.push(format!(
                "You are on `{current_branch}`, but this exercise expects final branch `{}`.",
                context.expected_branch
            ));
        }
    }

    if !context.expected_branch_exists {
        hints.push(format!(
            "The expected branch `{}` is missing. Recreate or restore it before final validation.",
            context.expected_branch
        ));
    }

    if !context.working_tree_clean {
        hints.push(
            "The working tree is not clean. Finish, commit, stash, or discard changes before final validation."
                .to_string(),
        );
    }

    for file in &context.missing_expected_files {
        hints.push(format!(
            "Expected file `{file}` is missing. Restore it before final validation."
        ));
    }

    hints
}

fn exercise_hints(path: &Path, context: &HintContext) -> Vec<String> {
    match context.exercise.as_str() {
        "conflict-basic" => conflict_basic_hints(path, context),
        "revert-mistake" => revert_mistake_hints(path, context),
        "wrong-branch-commit" => wrong_branch_commit_hints(path, context),
        "stash-switch" => stash_switch_hints(path, context),
        "cherry-pick-basic" => cherry_pick_basic_hints(path, context),
        "detached-head-recovery" => detached_head_recovery_hints(path, context),
        _ => Vec::new(),
    }
}

fn conflict_basic_hints(path: &Path, context: &HintContext) -> Vec<String> {
    let mut hints = Vec::new();
    if context.has_conflict_markers {
        hints.push(
            "For conflict-basic, remove conflict markers from `app.txt` and keep both the headline and CTA changes."
                .to_string(),
        );
    } else if context.current_branch.as_deref() == Some("main")
        && git::branch_exists(path, CONFLICT_FEATURE_BRANCH).unwrap_or(false)
        && !file_contains(path, "app.txt", EXPECTED_CTA)
    {
        hints.push(
            "Merge `feature/landing-copy` into `main`, then resolve `app.txt` if Git reports a conflict."
                .to_string(),
        );
    }
    hints
}

fn revert_mistake_hints(path: &Path, _context: &HintContext) -> Vec<String> {
    if file_contains(path, CONFIG_FILE, UNSAFE_VALUE) {
        vec![
            "The unsafe config value is still present. Find the bad commit and make a history-preserving fix."
                .to_string(),
            "The bad commit should remain visible in history after the fix.".to_string(),
        ]
    } else {
        Vec::new()
    }
}

fn wrong_branch_commit_hints(path: &Path, _context: &HintContext) -> Vec<String> {
    let mut hints = Vec::new();
    if !git::branch_exists(path, WRONG_BRANCH_FEATURE).unwrap_or(false) {
        hints.push(
            "The feature branch is missing. Inspect branches before moving the work.".to_string(),
        );
    }
    if branch_file_contains(path, "main", ACCIDENTAL_FILE, ACCIDENTAL_CONTENT) {
        hints.push(
            "The profile work still appears on `main`. Move it to `feature/profile-page`, then restore `main`."
                .to_string(),
        );
    }
    hints
}

fn stash_switch_hints(path: &Path, context: &HintContext) -> Vec<String> {
    let mut hints = Vec::new();
    if context.current_branch.as_deref() == Some("main") && !context.working_tree_clean {
        hints.push(
            "You have local work on `main`. Preserve it before switching to `feature/settings-copy`."
                .to_string(),
        );
    }
    if !git::branch_exists(path, STASH_FEATURE_BRANCH).unwrap_or(false) {
        hints.push(
            "The feature branch is missing. Check available branches before continuing."
                .to_string(),
        );
    }
    if branch_file_contains(path, "main", STASH_APP_FILE, SETTINGS_COPY) {
        hints.push(
            "The feature settings copy appears on `main`. Keep feature work on `feature/settings-copy` and leave `main` clean."
                .to_string(),
        );
    }
    hints
}

fn cherry_pick_basic_hints(path: &Path, context: &HintContext) -> Vec<String> {
    let mut hints = Vec::new();
    if context.current_branch.as_deref() != Some(RELEASE_BRANCH) {
        hints.push("Switch to `release/current` before applying the target bugfix.".to_string());
    }
    if !branch_file_contains(path, RELEASE_BRANCH, CHERRY_APP_FILE, BUGFIX_CONTENT) {
        hints.push(
            "The release branch is missing the target bugfix. Locate `Fix empty checkout cart` on `support/legacy-fix` and apply only that change."
                .to_string(),
        );
    }
    if branch_file_contains(path, RELEASE_BRANCH, LEGACY_FILE, LEGACY_CONTENT) {
        hints.push(
            "Legacy-only support content is present on `release/current`. Remove unrelated support changes from the release branch."
                .to_string(),
        );
    }
    if !git::branch_exists(path, SUPPORT_BRANCH).unwrap_or(false) {
        hints.push(
            "The support branch is missing. Restore `support/legacy-fix` before checking."
                .to_string(),
        );
    }
    hints
}

fn detached_head_recovery_hints(path: &Path, context: &HintContext) -> Vec<String> {
    let mut hints = Vec::new();
    if context.is_detached {
        hints.push(
            "Create `recovery/detached-work` from the current detached commit to keep the recovered work reachable."
                .to_string(),
        );
    }
    if context.current_branch.as_deref() == Some("main") {
        hints.push(
            "You are on `main`. Check whether `recovery/detached-work` exists and contains the recovered note."
                .to_string(),
        );
    }
    if !branch_file_contains(path, RECOVERY_BRANCH, RECOVERED_FILE, RECOVERED_CONTENT) {
        hints.push(
            "The recovery branch does not yet contain the recovered note. Preserve the detached work on `recovery/detached-work`."
                .to_string(),
        );
    }
    hints
}

fn active_operation_name(path: &Path) -> Option<String> {
    let git_dir = path.join(".git");
    [
        ("merge", "MERGE_HEAD"),
        ("rebase", "REBASE_HEAD"),
        ("cherry-pick", "CHERRY_PICK_HEAD"),
        ("revert", "REVERT_HEAD"),
    ]
    .iter()
    .find_map(|(operation, file)| {
        git_dir
            .join(file)
            .exists()
            .then(|| (*operation).to_string())
    })
    .or_else(|| {
        (git_dir.join("rebase-merge").exists() || git_dir.join("rebase-apply").exists())
            .then(|| "rebase".to_string())
    })
}

fn has_conflict_markers(path: &Path, files: &[String]) -> bool {
    files.iter().any(|file| {
        fs::read_to_string(path.join(file))
            .map(|content| {
                content.contains("<<<<<<<")
                    || content.contains("=======")
                    || content.contains(">>>>>>>")
            })
            .unwrap_or(false)
    })
}

fn file_contains(path: &Path, file: &str, expected: &str) -> bool {
    fs::read_to_string(path.join(file))
        .map(|content| normalize(&content).contains(expected))
        .unwrap_or(false)
}

fn branch_file_contains(path: &Path, branch: &str, file: &str, expected: &str) -> bool {
    git::file_content_at_branch(path, branch, file)
        .map(|content| {
            content
                .map(|value| normalize(&value).contains(expected))
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

fn normalize(value: &str) -> String {
    value.replace("\r\n", "\n").replace('\r', "\n")
}
