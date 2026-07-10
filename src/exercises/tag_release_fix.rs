use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::exercises::{generated_readme, metadata, Exercise};
use crate::git;
use crate::state::{write_state, BranchDojoState};

pub const RELEASE_TAG: &str = "v1.0.0";
pub const VERSION_FILE: &str = "VERSION";
pub const APP_FILE: &str = "app.txt";
pub const NOTES_FILE: &str = "release-notes.md";
pub const VERSION_CONTENT: &str = "1.0.0";
pub const RELEASE_NOTES_CONTENT: &str = "BranchDojo sample app v1.0.0";
pub const BAD_BLOCKER_CONTENT: &str = "release_blocker=true";
pub const FIXED_BLOCKER_CONTENT: &str = "release_blocker=false";
pub const RELEASE_READY_CONTENT: &str = "release_ready=true";
pub const INITIAL_COMMIT_MESSAGE: &str = "Initialize release project";
pub const RELEASE_CANDIDATE_COMMIT_MESSAGE: &str = "Prepare v1.0.0 release";
pub const FIX_COMMIT_MESSAGE: &str = "Fix release blocker";
pub const PREFERRED_TAG_MESSAGE: &str = "Release v1.0.0";

pub static EXERCISE: Exercise = Exercise {
    metadata: &metadata::TAG_RELEASE_FIX,
    goal: "Practice correcting a local release tag after a release-blocking fix.",
    hints: &[
        "Inspect history with `git log --oneline --decorate --graph --all`.",
        "Inspect the current tag target with `git show v1.0.0`.",
        "Compare the tagged content with `main`.",
        "Move or recreate `v1.0.0` so it points to the fixed release commit.",
        "Prefer an annotated tag for the release.",
        "Keep the working tree clean and run `branchdojo check --path .`.",
    ],
    setup,
};

pub fn setup(path: &Path) -> AppResult<()> {
    git::init_repo(path)?;
    git::set_local_identity(path)?;
    fs::write(path.join(VERSION_FILE), "0.9.0\n")
        .map_err(|error| AppError::io("Could not write VERSION.", error))?;
    write_app(
        path,
        "\
release_blocker=true
release_ready=false
",
    )?;
    git::add_all(path)?;
    git::commit(path, INITIAL_COMMIT_MESSAGE)?;

    fs::write(path.join(VERSION_FILE), format!("{VERSION_CONTENT}\n"))
        .map_err(|error| AppError::io("Could not write VERSION.", error))?;
    fs::write(path.join(NOTES_FILE), format!("{RELEASE_NOTES_CONTENT}\n"))
        .map_err(|error| AppError::io("Could not write release-notes.md.", error))?;
    write_state(
        path,
        &BranchDojoState::new_with_branch(
            EXERCISE.metadata.name,
            metadata::expected_final_branch_for(EXERCISE.metadata.name)
                .expect("exercise metadata should define expected final branch"),
            vec![VERSION_FILE.to_string(), APP_FILE.to_string()],
        ),
    )?;
    fs::write(
        path.join("README.branchdojo.md"),
        generated_readme(
            EXERCISE.metadata.title,
            EXERCISE.goal,
            "main",
            "VERSION, release-notes.md, app.txt",
            "main",
            "`v1.0.0` was tagged before a release-blocking fix landed.",
            "Inspect the release tag and history, then move or recreate the local `v1.0.0` tag so it points to the fixed release commit. Prefer an annotated tag, end on `main`, and leave the working tree clean.",
        ),
    )
    .map_err(|error| AppError::io("Could not write README.branchdojo.md.", error))?;
    git::add_all(path)?;
    git::commit(path, RELEASE_CANDIDATE_COMMIT_MESSAGE)?;
    git::run_git(
        path,
        ["tag", "-a", RELEASE_TAG, "-m", PREFERRED_TAG_MESSAGE],
    )?;

    write_app(
        path,
        "\
release_blocker=false
release_ready=true
",
    )?;
    git::add_all(path)?;
    git::commit(path, FIX_COMMIT_MESSAGE)?;
    Ok(())
}

fn write_app(path: &Path, content: &str) -> AppResult<()> {
    fs::write(path.join(APP_FILE), content)
        .map_err(|error| AppError::io("Could not write app.txt.", error))
}
