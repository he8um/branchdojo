use std::fs;
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::exercises::{generated_readme, Exercise};
use crate::git;
use crate::state::{write_state, BranchDojoState};

pub const RECOVERY_BRANCH: &str = "recovery/detached-work";
pub const RECOVERED_FILE: &str = "recovered-note.txt";
pub const RECOVERED_CONTENT: &str = "Recovered detached HEAD work";
pub const DETACHED_COMMIT_MESSAGE: &str = "Add detached work note";

const APP_FILE: &str = "app.txt";

pub static EXERCISE: Exercise = Exercise {
    id: "detached-head-recovery",
    title: "Detached HEAD Recovery",
    difficulty: "Intermediate",
    estimated_time: "10 to 15 minutes",
    skills: &[
        "Detached HEAD recovery",
        "Branch creation",
        "Reachability inspection",
    ],
    description: "Recover useful work committed while HEAD is detached.",
    goal: "Practice making detached HEAD work reachable from a named branch.",
    hints: &[
        "Run `git status` and notice the detached HEAD state.",
        "Inspect recent commits with `git log --oneline --decorate --graph --all`.",
        "Create `recovery/detached-work` so it points at the detached work.",
        "Make sure you are on `recovery/detached-work`.",
        "Confirm `recovered-note.txt` contains the recovered work.",
        "Run `branchdojo check --path .`.",
    ],
    setup,
};

pub fn setup(path: &Path) -> AppResult<()> {
    git::init_repo(path)?;
    git::set_local_identity(path)?;
    write_app(path, "BranchDojo demo app\n")?;
    write_state(
        path,
        &BranchDojoState::new_with_branch(
            EXERCISE.id,
            RECOVERY_BRANCH,
            vec![RECOVERED_FILE.to_string()],
        ),
    )?;
    fs::write(
        path.join("README.branchdojo.md"),
        generated_readme(
            EXERCISE.title,
            EXERCISE.goal,
            "main, detached HEAD",
            "app.txt, recovered-note.txt",
            "detached HEAD",
            "useful work was committed while HEAD was detached.",
            "Create or update `recovery/detached-work` so the detached work is reachable from that branch, then leave the repository clean on that branch.",
        ),
    )
    .map_err(|error| AppError::io("Could not write README.branchdojo.md.", error))?;
    git::add_all(path)?;
    git::commit(path, "Add demo app baseline")?;

    let baseline = git::run_git(path, ["rev-parse", "HEAD"])?
        .trim()
        .to_string();
    write_app(path, "BranchDojo demo app\nMain branch follow-up change\n")?;
    git::add_all(path)?;
    git::commit(path, "Add main follow-up change")?;

    git::run_git(path, ["checkout", &baseline])?;
    fs::write(path.join(RECOVERED_FILE), format!("{RECOVERED_CONTENT}\n"))
        .map_err(|error| AppError::io("Could not write recovered-note.txt.", error))?;
    git::add_all(path)?;
    git::commit(path, DETACHED_COMMIT_MESSAGE)?;
    Ok(())
}

fn write_app(path: &Path, content: &str) -> AppResult<()> {
    fs::write(path.join(APP_FILE), content)
        .map_err(|error| AppError::io("Could not write app.txt.", error))
}
