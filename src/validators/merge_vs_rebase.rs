use std::path::Path;

use crate::error::AppResult;
use crate::exercises::merge_vs_rebase::{
    CHECKOUT_FILE, CHECKOUT_TRUST_COPY, CHECKOUT_UPDATE_COMMIT_MESSAGE, FEATURE_BRANCH,
    PRICING_FAQ_COMMIT_MESSAGE, PRICING_FAQ_COPY, PRICING_FAQ_TITLE, PRICING_FILE,
    PRICING_HEADLINE, PRICING_HEADLINE_COMMIT_MESSAGE, PRICING_TITLE,
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
        common::branch_exists(path, FEATURE_BRANCH),
        common::branch_file_contains(
            path,
            "main",
            PRICING_FILE,
            "main_pricing_title",
            PRICING_TITLE,
            "Main has pricing title",
        ),
        common::branch_file_contains(
            path,
            "main",
            PRICING_FILE,
            "main_pricing_headline",
            PRICING_HEADLINE,
            "Main has pricing headline",
        ),
        common::branch_file_contains(
            path,
            "main",
            PRICING_FILE,
            "main_pricing_faq_title",
            PRICING_FAQ_TITLE,
            "Main has pricing FAQ title",
        ),
        common::branch_file_contains(
            path,
            "main",
            PRICING_FILE,
            "main_pricing_faq_copy",
            PRICING_FAQ_COPY,
            "Main has pricing FAQ copy",
        ),
        common::branch_file_contains(
            path,
            "main",
            CHECKOUT_FILE,
            "main_checkout_trust_copy",
            CHECKOUT_TRUST_COPY,
            "Main has checkout trust copy",
        ),
        common::branch_file_contains(
            path,
            FEATURE_BRANCH,
            PRICING_FILE,
            "feature_pricing_headline",
            PRICING_HEADLINE,
            "Feature branch has pricing headline",
        ),
        common::branch_file_contains(
            path,
            FEATURE_BRANCH,
            PRICING_FILE,
            "feature_pricing_faq_copy",
            PRICING_FAQ_COPY,
            "Feature branch has pricing FAQ copy",
        ),
    ];

    let required_checks_pass = checks
        .iter()
        .filter(|check| check.severity == Severity::Required)
        .all(|check| check.status == CheckStatus::Passed);
    checks.push(CheckResult::warning(
        "merge_commit_integration",
        "Merge commit integration check",
        required_checks_pass && merge_commit_exists(path),
    ));
    checks.push(CheckResult::warning(
        "feature_branch_updated_with_main",
        "Feature branch update check",
        required_checks_pass && !feature_branch_has_checkout_update(path),
    ));
    checks.push(CheckResult::warning(
        "feature_commits_reachable",
        "Feature commit subject check",
        required_checks_pass && !feature_commits_reachable(path),
    ));
    checks.push(CheckResult::warning(
        "linear_history_order",
        "Linear history order check",
        required_checks_pass && !history_order_is_preferred(path),
    ));

    Ok(ValidationResult::new(
        &state.exercise,
        checks,
        vec![
            "Update feature/pricing-copy with main, integrate the pricing feature into main, preserve checkout.txt and pricing.txt, then run branchdojo check --path . again.".to_string(),
            "If the result is a warning, inspect the graph and try a cleaner linear integration.".to_string(),
        ],
    ))
}

fn merge_commit_exists(path: &Path) -> bool {
    git::run_git(path, ["log", "main", "--merges", "--format=%H"])
        .map(|log| !log.trim().is_empty())
        .unwrap_or(false)
}

fn feature_branch_has_checkout_update(path: &Path) -> bool {
    git::file_content_at_branch(path, FEATURE_BRANCH, CHECKOUT_FILE)
        .map(|content| {
            content
                .map(|value| common::normalize(&value).contains(CHECKOUT_TRUST_COPY))
                .unwrap_or(false)
        })
        .unwrap_or(false)
}

fn feature_commits_reachable(path: &Path) -> bool {
    let log = git::run_git(path, ["log", "main", "--format=%s"]).unwrap_or_default();
    log.lines()
        .any(|line| line == PRICING_HEADLINE_COMMIT_MESSAGE)
        && log.lines().any(|line| line == PRICING_FAQ_COMMIT_MESSAGE)
}

fn history_order_is_preferred(path: &Path) -> bool {
    let log = git::run_git(path, ["log", "main", "--reverse", "--format=%s"]).unwrap_or_default();
    let subjects: Vec<_> = log.lines().collect();
    let checkout = position(&subjects, CHECKOUT_UPDATE_COMMIT_MESSAGE);
    let headline = position(&subjects, PRICING_HEADLINE_COMMIT_MESSAGE);
    let faq = position(&subjects, PRICING_FAQ_COMMIT_MESSAGE);
    matches!(
        (checkout, headline, faq),
        (Some(checkout), Some(headline), Some(faq)) if checkout < headline && headline < faq
    )
}

fn position(subjects: &[&str], needle: &str) -> Option<usize> {
    subjects.iter().position(|subject| *subject == needle)
}
