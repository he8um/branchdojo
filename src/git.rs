use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

use crate::error::{AppError, AppResult};

pub fn ensure_git_available() -> AppResult<()> {
    let output = Command::new("git").arg("--version").output().map_err(|_| {
        AppError::new(
            "BD001",
            "Git is not installed or not available in PATH.",
            "BranchDojo needs the local Git binary to create and validate exercises.",
            "Install Git and run the command again.",
        )
    })?;
    if output.status.success() {
        Ok(())
    } else {
        Err(AppError::new(
            "BD001",
            "Git is not installed or not available in PATH.",
            "BranchDojo needs the local Git binary to create and validate exercises.",
            "Install Git and run the command again.",
        ))
    }
}

pub fn init_repo(path: &Path) -> AppResult<()> {
    run_git(path, ["init", "-b", "main"]).map(|_| ())
}

pub fn set_local_identity(path: &Path) -> AppResult<()> {
    run_git(path, ["config", "user.name", "BranchDojo"])?;
    run_git(path, ["config", "user.email", "branchdojo@example.local"])?;
    Ok(())
}

pub fn checkout_new_branch(path: &Path, branch: &str) -> AppResult<()> {
    run_git(path, ["checkout", "-b", branch]).map(|_| ())
}

pub fn checkout_branch(path: &Path, branch: &str) -> AppResult<()> {
    run_git(path, ["checkout", branch]).map(|_| ())
}

pub fn add_all(path: &Path) -> AppResult<()> {
    run_git(path, ["add", "."]).map(|_| ())
}

pub fn commit(path: &Path, message: &str) -> AppResult<()> {
    run_git(path, ["commit", "-m", message]).map(|_| ())
}

pub fn status_porcelain(path: &Path) -> AppResult<String> {
    run_git(path, ["status", "--porcelain"])
}

pub fn current_branch(path: &Path) -> AppResult<String> {
    run_git(path, ["branch", "--show-current"]).map(|value| value.trim().to_string())
}

pub fn branch_exists(path: &Path, branch: &str) -> AppResult<bool> {
    let result = Command::new("git")
        .current_dir(path)
        .args(["rev-parse", "--verify", "--quiet", branch])
        .output()
        .map_err(|error| AppError::io("Could not run git.", error))?;
    Ok(result.status.success())
}

pub fn file_content_at_branch(path: &Path, branch: &str, file: &str) -> AppResult<Option<String>> {
    let spec = format!("{branch}:{file}");
    let output = Command::new("git")
        .current_dir(path)
        .args(["show", &spec])
        .output()
        .map_err(|error| AppError::io("Could not run git.", error))?;
    if output.status.success() {
        Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
    } else {
        Ok(None)
    }
}

pub fn log_contains_message(path: &Path, message: &str) -> AppResult<bool> {
    let log = run_git(path, ["log", "--format=%s"])?;
    Ok(log.lines().any(|line| line == message))
}

pub fn commit_count_after_message(path: &Path, message: &str) -> AppResult<usize> {
    let log = run_git(path, ["log", "--format=%H%x00%s"])?;
    let Some(hash) = log.lines().find_map(|line| {
        let (hash, subject) = line.split_once('\0')?;
        if subject == message {
            Some(hash.to_string())
        } else {
            None
        }
    }) else {
        return Ok(0);
    };
    let range = format!("{hash}..HEAD");
    let count = run_git(path, ["rev-list", "--count", &range])?;
    Ok(count.trim().parse().unwrap_or(0))
}

pub fn merge_commit_exists(path: &Path) -> AppResult<bool> {
    let log = run_git(path, ["log", "--merges", "--format=%H"])?;
    Ok(!log.trim().is_empty())
}

pub fn active_operation(path: &Path) -> bool {
    let git_dir = path.join(".git");
    ["MERGE_HEAD", "REBASE_HEAD", "CHERRY_PICK_HEAD", "REVERT_HEAD"]
        .iter()
        .any(|file| git_dir.join(file).exists())
}

pub fn run_git<I, S>(path: &Path, args: I) -> AppResult<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new("git")
        .current_dir(path)
        .args(args)
        .output()
        .map_err(|error| AppError::io("Could not run git.", error))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(AppError::git(&String::from_utf8_lossy(&output.stderr)))
    }
}
