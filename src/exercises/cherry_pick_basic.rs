use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::exercises::{generated_readme, metadata, Exercise};
use crate::git;
use crate::state::{write_state, BranchDojoState};

pub const SUPPORT_BRANCH: &str = "support/legacy-fix";
pub const RELEASE_BRANCH: &str = "release/current";
pub const APP_FILE: &str = "app.txt";
pub const LEGACY_FILE: &str = "legacy.txt";
pub const BUGFIX_CONTENT: &str = "Fix: handle empty checkout cart";
pub const LEGACY_CONTENT: &str = "Legacy support mode enabled";
pub const BUGFIX_COMMIT_MESSAGE: &str = "Fix empty checkout cart";
pub const LEGACY_COMMIT_MESSAGE: &str = "Enable legacy support mode";

pub static EXERCISE: Exercise = Exercise {
    metadata: &metadata::CHERRY_PICK_BASIC,
    goal: "Practice moving a single useful commit onto the intended release branch.",
    hints: &[
        "Inspect the graph with `git log --oneline --decorate --graph --all`.",
        "Find the commit named `Fix empty checkout cart` on `support/legacy-fix`.",
        "Make sure you are on `release/current`.",
        "Apply only that bugfix commit.",
        "Confirm legacy-only content is absent from `release/current`.",
        "Run `branchdojo check --path .`.",
    ],
    setup,
};

pub fn setup(path: &Path) -> AppResult<()> {
    git::init_repo(path)?;
    git::set_local_identity(path)?;
    write_app(
        path,
        "\
Checkout: standard cart flow
",
    )?;
    git::add_all(path)?;
    git::commit(path, "Add checkout app baseline")?;

    write_state(
        path,
        &BranchDojoState::new_with_branch(
            EXERCISE.metadata.name,
            metadata::expected_final_branch_for(EXERCISE.metadata.name)
                .expect("exercise metadata should define expected final branch"),
            vec![APP_FILE.to_string()],
        ),
    )?;
    fs::write(
        path.join("README.branchdojo.md"),
        generated_readme(
            EXERCISE.metadata.title,
            EXERCISE.goal,
            "main, support/legacy-fix, release/current",
            "app.txt, legacy.txt",
            "release/current",
            "a useful checkout bugfix exists on a support branch beside unrelated legacy work.",
            "Find the `Fix empty checkout cart` commit on `support/legacy-fix` and apply only that fix to `release/current`.",
        ),
    )
    .map_err(|error| AppError::io("Could not write README.branchdojo.md.", error))?;
    git::add_all(path)?;
    git::commit(path, "Add BranchDojo exercise instructions")?;

    git::checkout_new_branch(path, RELEASE_BRANCH)?;
    git::checkout_branch(path, "main")?;
    git::checkout_new_branch(path, SUPPORT_BRANCH)?;

    fs::write(path.join(LEGACY_FILE), format!("{LEGACY_CONTENT}\n"))
        .map_err(|error| AppError::io("Could not write legacy.txt.", error))?;
    git::add_all(path)?;
    git::commit(path, LEGACY_COMMIT_MESSAGE)?;

    write_app(
        path,
        "\
Checkout: standard cart flow
Fix: handle empty checkout cart
",
    )?;
    git::add_all(path)?;
    git::commit(path, BUGFIX_COMMIT_MESSAGE)?;

    git::checkout_branch(path, RELEASE_BRANCH)?;
    Ok(())
}

fn write_app(path: &Path, content: &str) -> AppResult<()> {
    fs::write(path.join(APP_FILE), content)
        .map_err(|error| AppError::io("Could not write app.txt.", error))
}
