use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::exercises::{generated_readme, metadata, Exercise};
use crate::git;
use crate::state::{write_state, BranchDojoState};

pub const FEATURE_BRANCH: &str = "feature/pricing-copy";
pub const PRICING_FILE: &str = "pricing.txt";
pub const CHECKOUT_FILE: &str = "checkout.txt";
pub const PRICING_TITLE: &str = "Pricing";
pub const PRICING_HEADLINE: &str = "Simple plans for growing teams.";
pub const PRICING_FAQ_TITLE: &str = "Frequently asked questions";
pub const PRICING_FAQ_COPY: &str = "You can change plans at any time.";
pub const CHECKOUT_TITLE: &str = "Secure checkout";
pub const CHECKOUT_TRUST_COPY: &str = "Your payment is protected.";
pub const CHECKOUT_UPDATE_COMMIT_MESSAGE: &str = "Update checkout trust copy";
pub const PRICING_HEADLINE_COMMIT_MESSAGE: &str = "Add pricing page headline";
pub const PRICING_FAQ_COMMIT_MESSAGE: &str = "Add pricing FAQ copy";

pub static EXERCISE: Exercise = Exercise {
    metadata: &metadata::MERGE_VS_REBASE,
    goal: "Practice updating and integrating a feature branch with a clean history.",
    hints: &[
        "Inspect history with `git log --oneline --decorate --graph --all`.",
        "Compare branches with `git diff main..feature/pricing-copy`.",
        "Bring `feature/pricing-copy` up to date with `main`.",
        "Integrate the feature into `main` with a clean linear history if practical.",
        "Preserve both the checkout trust copy and the pricing feature copy.",
        "Keep the working tree clean and run `branchdojo check --path .`.",
    ],
    setup,
};

pub fn setup(path: &Path) -> AppResult<()> {
    git::init_repo(path)?;
    git::set_local_identity(path)?;
    write_pricing(path, "Pricing\nStarter plan available.\n")?;
    write_checkout(path, "Checkout\nStandard payment flow.\n")?;
    write_state(
        path,
        &BranchDojoState::new_with_branch(
            EXERCISE.metadata.name,
            metadata::expected_final_branch_for(EXERCISE.metadata.name)
                .expect("exercise metadata should define expected final branch"),
            vec![PRICING_FILE.to_string(), CHECKOUT_FILE.to_string()],
        ),
    )?;
    fs::write(
        path.join("README.branchdojo.md"),
        generated_readme(
            EXERCISE.metadata.title,
            EXERCISE.goal,
            "main, feature/pricing-copy",
            "pricing.txt, checkout.txt",
            "feature/pricing-copy",
            "`main` received checkout trust copy after the feature branch was created.",
            "Bring `feature/pricing-copy` up to date with `main`, integrate it into `main`, preserve both files' expected content, prefer a clean linear integration, and leave the repository clean on `main`.",
        ),
    )
    .map_err(|error| AppError::io("Could not write README.branchdojo.md.", error))?;
    git::add_all(path)?;
    git::commit(path, "Initialize pricing project")?;

    git::checkout_new_branch(path, FEATURE_BRANCH)?;

    git::checkout_branch(path, "main")?;
    write_checkout(path, &final_checkout_content())?;
    git::add_all(path)?;
    git::commit(path, CHECKOUT_UPDATE_COMMIT_MESSAGE)?;

    git::checkout_branch(path, FEATURE_BRANCH)?;
    write_pricing(
        path,
        "\
Pricing
Simple plans for growing teams.
",
    )?;
    git::add_all(path)?;
    git::commit(path, PRICING_HEADLINE_COMMIT_MESSAGE)?;

    write_pricing(path, &final_pricing_content())?;
    git::add_all(path)?;
    git::commit(path, PRICING_FAQ_COMMIT_MESSAGE)?;
    Ok(())
}

pub fn final_pricing_content() -> String {
    format!("{PRICING_TITLE}\n{PRICING_HEADLINE}\n{PRICING_FAQ_TITLE}\n{PRICING_FAQ_COPY}\n")
}

pub fn final_checkout_content() -> String {
    format!("{CHECKOUT_TITLE}\n{CHECKOUT_TRUST_COPY}\n")
}

fn write_pricing(path: &Path, content: &str) -> AppResult<()> {
    fs::write(path.join(PRICING_FILE), content)
        .map_err(|error| AppError::io("Could not write pricing.txt.", error))
}

fn write_checkout(path: &Path, content: &str) -> AppResult<()> {
    fs::write(path.join(CHECKOUT_FILE), content)
        .map_err(|error| AppError::io("Could not write checkout.txt.", error))
}
