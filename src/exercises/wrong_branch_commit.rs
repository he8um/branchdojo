use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::exercises::{generated_readme, metadata, Exercise};
use crate::git;
use crate::state::{write_state, BranchDojoState};

pub const FEATURE_BRANCH: &str = "feature/profile-page";
pub const ACCIDENTAL_FILE: &str = "profile.md";
pub const ACCIDENTAL_CONTENT: &str = "Profile page draft";

pub static EXERCISE: Exercise = Exercise {
    metadata: &metadata::WRONG_BRANCH_COMMIT,
    goal: "Practice moving committed work to the right branch while keeping main clean.",
    hints: &[
        "Run `git branch`.",
        "Run `git log --oneline --decorate --graph --all`.",
        "Identify which commit belongs on the feature branch.",
        "Move the work to `feature/profile-page`.",
        "Restore `main` to the correct state.",
        "Run `branchdojo check --path .`.",
    ],
    setup,
};

pub fn setup(path: &Path) -> AppResult<()> {
    git::init_repo(path)?;
    git::set_local_identity(path)?;
    fs::write(path.join("README.md"), "# Demo App\n")
        .map_err(|error| AppError::io("Could not write README.md.", error))?;
    git::add_all(path)?;
    git::commit(path, "Add demo app readme")?;

    write_state(
        path,
        &BranchDojoState::new(EXERCISE.metadata.name, vec![ACCIDENTAL_FILE.to_string()]),
    )?;
    fs::write(
        path.join("README.branchdojo.md"),
        generated_readme(
            EXERCISE.metadata.title,
            EXERCISE.goal,
            "main, feature/profile-page",
            "README.md, profile.md",
            "main",
            "profile page work was committed on main instead of the feature branch.",
            "Move the profile page work to `feature/profile-page` and restore `main` so it does not contain that work.",
        ),
    )
    .map_err(|error| AppError::io("Could not write README.branchdojo.md.", error))?;
    git::add_all(path)?;
    git::commit(path, "Add BranchDojo exercise instructions")?;

    git::checkout_new_branch(path, FEATURE_BRANCH)?;
    git::checkout_branch(path, "main")?;

    fs::write(
        path.join(ACCIDENTAL_FILE),
        format!("{ACCIDENTAL_CONTENT}\nOwner: feature/profile-page\n"),
    )
    .map_err(|error| AppError::io("Could not write profile.md.", error))?;
    git::add_all(path)?;
    git::commit(path, "Add profile page draft")?;
    Ok(())
}
