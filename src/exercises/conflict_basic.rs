use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::exercises::{generated_readme, Exercise};
use crate::git;
use crate::state::{write_state, BranchDojoState};

pub const EXPECTED_HEADLINE: &str = "Headline: Build better Git habits";
pub const EXPECTED_CTA: &str = "CTA: Start practicing";
pub const FEATURE_BRANCH: &str = "feature/landing-copy";
const APP_FILE: &str = "app.txt";

pub static EXERCISE: Exercise = Exercise {
    id: "conflict-basic",
    title: "Basic Merge Conflict",
    difficulty: "Beginner",
    estimated_time: "5 to 10 minutes",
    skills: &["Branch awareness", "Merge conflict resolution", "Clean merge completion"],
    description: "Resolve a small merge conflict and keep both intended changes.",
    goal: "Practice resolving a real merge conflict on main.",
    hints: &[
        "Start with `git status`.",
        "Make sure you are on `main`.",
        "Merge `feature/landing-copy`.",
        "Open `app.txt` and remove conflict markers.",
        "Keep both required changes.",
        "Add and commit the resolved file.",
        "Run `branchdojo check --path .`.",
    ],
    setup,
};

pub fn setup(path: &Path) -> AppResult<()> {
    git::init_repo(path)?;
    git::set_local_identity(path)?;
    write_file(path, "Hero: Welcome | CTA: Learn more\n")?;
    git::add_all(path)?;
    git::commit(path, "Add initial landing copy")?;

    git::checkout_new_branch(path, FEATURE_BRANCH)?;
    write_file(path, "Hero: Welcome | CTA: Start practicing\n")?;
    git::add_all(path)?;
    git::commit(path, "Update landing CTA")?;

    git::checkout_branch(path, "main")?;
    write_file(path, "Headline: Build better Git habits | CTA: Learn more\n")?;
    git::add_all(path)?;
    git::commit(path, "Update landing headline")?;

    write_state(
        path,
        &BranchDojoState::new(EXERCISE.id, vec![APP_FILE.to_string()]),
    )?;
    fs::write(
        path.join("README.branchdojo.md"),
        generated_readme(
            EXERCISE.title,
            EXERCISE.goal,
            "main, feature/landing-copy",
            "app.txt",
            "main",
            "main and feature changed the same landing-copy line.",
            "Merge `feature/landing-copy` into `main` and resolve `app.txt` so it keeps the updated headline and CTA.",
        ),
    )
    .map_err(|error| AppError::io("Could not write README.branchdojo.md.", error))?;
    Ok(())
}

fn write_file(path: &Path, content: &str) -> AppResult<()> {
    fs::write(path.join(APP_FILE), content)
        .map_err(|error| AppError::io("Could not write app.txt.", error))
}
