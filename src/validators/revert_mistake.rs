use std::path::Path;

use crate::error::AppResult;
use crate::exercises::revert_mistake::{
    BAD_COMMIT_MESSAGE, CONFIG_FILE, SAFE_VALUE, UNSAFE_VALUE,
};
use crate::git;
use crate::result::{CheckResult, ValidationResult};
use crate::state::BranchDojoState;
use crate::validators::common;

pub fn validate(path: &Path, state: &BranchDojoState) -> AppResult<ValidationResult> {
    let bad_commit_exists = git::log_contains_message(path, BAD_COMMIT_MESSAGE).unwrap_or(false);
    let fix_commit_after_bad = git::commit_count_after_message(path, BAD_COMMIT_MESSAGE)
        .map(|count| count >= 1)
        .unwrap_or(false);
    let revert_style = git::log_contains_message(path, &format!("Revert \"{BAD_COMMIT_MESSAGE}\""))
        .unwrap_or(false);

    let mut checks = vec![
        common::metadata_exists(path),
        common::git_directory_exists(path),
        common::current_branch_main(path),
        common::working_tree_clean(path),
        common::no_active_git_operation(path),
        common::file_exists(path, CONFIG_FILE),
        common::file_not_contains(
            path,
            CONFIG_FILE,
            "unsafe_config",
            UNSAFE_VALUE,
            "Unsafe config is absent",
        ),
        common::file_contains(
            path,
            CONFIG_FILE,
            "safe_config",
            SAFE_VALUE,
            "Safe config exists",
        ),
    ];
    checks.push(CheckResult::required(
        "bad_commit_still_exists",
        "Bad commit still exists",
        bad_commit_exists,
    ));
    checks.push(CheckResult::required(
        "fix_commit_after_bad_commit",
        "Fix commit exists after bad commit",
        fix_commit_after_bad,
    ));
    checks.push(CheckResult::warning(
        "revert_style_detected",
        "Final state is valid, but no revert-style commit was detected",
        !revert_style,
    ));

    Ok(ValidationResult::new(
        &state.exercise,
        checks,
        vec![
            "Restore config.txt to the safe value while keeping the bad commit in history, then run branchdojo check --path . again.".to_string(),
            "If the result is a warning, try solving this again using git revert.".to_string(),
        ],
    ))
}
