use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::exercises::{generated_readme, metadata, Exercise};
use crate::git;
use crate::state::{write_state, BranchDojoState};

pub const FEATURE_BRANCH: &str = "feature/settings-copy";
pub const APP_FILE: &str = "app.txt";
pub const SETTINGS_COPY: &str = "Settings: Save preferences with confidence.";
pub const PRESERVED_WORK: &str = "Local note: Keep dark mode feedback for follow-up.";

pub static EXERCISE: Exercise = Exercise {
    metadata: &metadata::STASH_SWITCH,
    goal: "Practice saving local work, completing feature branch work, and returning to a clean main branch.",
    hints: &[
        "Start with `git status`.",
        "Save the local work on `main` before switching branches.",
        "Switch to `feature/settings-copy`.",
        "Update `app.txt` with the expected settings copy and commit it.",
        "Return to `main` and restore the saved local work.",
        "Commit the preserved local work so the working tree is clean.",
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
Welcome: BranchDojo demo app
Settings: Preferences are ready.
",
    )?;
    git::add_all(path)?;
    git::commit(path, "Add demo app copy")?;

    write_state(
        path,
        &BranchDojoState::new(EXERCISE.metadata.name, vec![APP_FILE.to_string()]),
    )?;
    fs::write(
        path.join("README.branchdojo.md"),
        generated_readme(
            EXERCISE.metadata.title,
            EXERCISE.goal,
            "main, feature/settings-copy",
            "app.txt",
            "main",
            "main has uncommitted local work that conflicts with switching to the feature branch.",
            "Preserve the local work, switch to `feature/settings-copy`, update the settings copy, then return to `main` with the preserved work committed and a clean working tree.",
        ),
    )
    .map_err(|error| AppError::io("Could not write README.branchdojo.md.", error))?;
    git::add_all(path)?;
    git::commit(path, "Add BranchDojo exercise instructions")?;

    git::checkout_new_branch(path, FEATURE_BRANCH)?;
    write_app(
        path,
        "\
Welcome: BranchDojo demo app
Settings: Preferences need clearer copy.
",
    )?;
    git::add_all(path)?;
    git::commit(path, "Prepare settings copy branch")?;

    git::checkout_branch(path, "main")?;
    write_app(
        path,
        &format!(
            "\
Welcome: BranchDojo demo app
Settings: Preferences are ready.
{PRESERVED_WORK}
"
        ),
    )?;
    Ok(())
}

fn write_app(path: &Path, content: &str) -> AppResult<()> {
    fs::write(path.join(APP_FILE), content)
        .map_err(|error| AppError::io("Could not write app.txt.", error))
}
