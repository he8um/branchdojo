use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::exercises::{generated_readme, metadata, Exercise};
use crate::git;
use crate::state::{write_state, BranchDojoState};

pub const FEATURE_BRANCH: &str = "feature/profile-copy";
pub const PROFILE_FILE: &str = "profile.txt";
pub const DEBUG_FILE: &str = "debug.txt";
pub const PROFILE_TITLE: &str = "Profile page";
pub const PROFILE_WELCOME: &str = "Welcome to your profile.";
pub const PROFILE_MANAGE: &str = "Manage your account details here.";
pub const DEBUG_CONTENT: &str = "temporary debug notes";
pub const DRAFT_COMMIT_MESSAGE: &str = "Add profile copy draft";
pub const WIP_COMMIT_MESSAGE: &str = "WIP debug profile copy";
pub const TYPO_COMMIT_MESSAGE: &str = "Fix profile copy typo";
pub const POLISH_COMMIT_MESSAGE: &str = "Polish profile copy";

pub static EXERCISE: Exercise = Exercise {
    metadata: &metadata::INTERACTIVE_REBASE_BASIC,
    goal: "Practice cleaning a messy local feature branch before sharing it.",
    hints: &[
        "Inspect history with `git log --oneline --decorate --graph`.",
        "Use interactive rebase or another safe history-cleanup workflow.",
        "Remove, squash, or fix up noisy WIP/debug commits.",
        "Keep the final profile copy content in `profile.txt`.",
        "Confirm `debug.txt` is gone and the working tree is clean.",
        "Run `branchdojo check --path .`.",
    ],
    setup,
};

pub fn setup(path: &Path) -> AppResult<()> {
    git::init_repo(path)?;
    git::set_local_identity(path)?;
    write_profile(
        path,
        "\
Profile page
Baseline profile shell.
",
    )?;
    write_state(
        path,
        &BranchDojoState::new_with_branch(
            EXERCISE.metadata.name,
            metadata::expected_final_branch_for(EXERCISE.metadata.name)
                .expect("exercise metadata should define expected final branch"),
            vec![PROFILE_FILE.to_string()],
        ),
    )?;
    fs::write(
        path.join("README.branchdojo.md"),
        generated_readme(
            EXERCISE.metadata.title,
            EXERCISE.goal,
            "main, feature/profile-copy",
            "profile.txt, debug.txt",
            "feature/profile-copy",
            "the feature branch contains useful profile copy plus noisy WIP/debug history.",
            "Clean the feature branch before sharing it: preserve the final profile copy, remove debug/WIP work, end on `feature/profile-copy`, and leave the working tree clean.",
        ),
    )
    .map_err(|error| AppError::io("Could not write README.branchdojo.md.", error))?;
    git::add_all(path)?;
    git::commit(path, "Add profile baseline")?;

    git::checkout_new_branch(path, FEATURE_BRANCH)?;
    write_profile(
        path,
        "\
Profile draft
Welcome to your profile.
Manage your account detials here.
",
    )?;
    git::add_all(path)?;
    git::commit(path, DRAFT_COMMIT_MESSAGE)?;

    fs::write(path.join(DEBUG_FILE), format!("{DEBUG_CONTENT}\n"))
        .map_err(|error| AppError::io("Could not write debug.txt.", error))?;
    git::add_all(path)?;
    git::commit(path, WIP_COMMIT_MESSAGE)?;

    write_profile(
        path,
        "\
Profile draft
Welcome to your profile.
Manage your account details here.
",
    )?;
    git::add_all(path)?;
    git::commit(path, TYPO_COMMIT_MESSAGE)?;

    write_profile(
        path,
        "\
Profile page
Welcome to your profile.
Manage your account details here.
",
    )?;
    git::add_all(path)?;
    git::commit(path, POLISH_COMMIT_MESSAGE)?;
    Ok(())
}

fn write_profile(path: &Path, content: &str) -> AppResult<()> {
    fs::write(path.join(PROFILE_FILE), content)
        .map_err(|error| AppError::io("Could not write profile.txt.", error))
}
