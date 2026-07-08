use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::exercises::{generated_readme, metadata, Exercise};
use crate::git;
use crate::state::{write_state, BranchDojoState};

pub const CONFIG_FILE: &str = "config.txt";
pub const SAFE_VALUE: &str = "mode=safe";
pub const UNSAFE_VALUE: &str = "mode=unsafe";
pub const BAD_COMMIT_MESSAGE: &str = "Enable unsafe production mode";

pub static EXERCISE: Exercise = Exercise {
    metadata: &metadata::REVERT_MISTAKE,
    goal: "Practice fixing a bad commit without destroying history.",
    hints: &[
        "Run `git log --oneline`.",
        "Find the commit that introduced the unsafe config.",
        "Prefer a history-preserving recovery.",
        "Confirm `config.txt` contains the safe value.",
        "Confirm the bad commit still exists in history.",
        "Run `branchdojo check --path .`.",
    ],
    setup,
};

pub fn setup(path: &Path) -> AppResult<()> {
    git::init_repo(path)?;
    git::set_local_identity(path)?;
    write_config(path, "mode=safe\nfeature_flags=basic\n")?;
    git::add_all(path)?;
    git::commit(path, "Add safe production config")?;

    write_state(
        path,
        &BranchDojoState::new(EXERCISE.metadata.name, vec![CONFIG_FILE.to_string()]),
    )?;
    fs::write(
        path.join("README.branchdojo.md"),
        generated_readme(
            EXERCISE.metadata.title,
            EXERCISE.goal,
            "main",
            "config.txt",
            "main",
            "the latest commit changed production config to an unsafe value.",
            "Use a history-preserving fix so `config.txt` is safe again while the bad commit remains visible in history.",
        ),
    )
    .map_err(|error| AppError::io("Could not write README.branchdojo.md.", error))?;
    git::add_all(path)?;
    git::commit(path, "Add BranchDojo exercise instructions")?;

    write_config(path, "mode=safe\nfeature_flags=basic,search\n")?;
    git::add_all(path)?;
    git::commit(path, "Enable search feature flag")?;

    write_config(path, "mode=unsafe\nfeature_flags=basic,search\n")?;
    git::add_all(path)?;
    git::commit(path, BAD_COMMIT_MESSAGE)?;
    Ok(())
}

fn write_config(path: &Path, content: &str) -> AppResult<()> {
    fs::write(path.join(CONFIG_FILE), content)
        .map_err(|error| AppError::io("Could not write config.txt.", error))
}
