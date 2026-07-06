use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::error::{AppError, AppResult};
use crate::exercises;
use crate::state::{read_state, BranchDojoState};

pub fn ensure_safe_new_path(path: &Path) -> AppResult<()> {
    ensure_path_not_unsafe(path)?;
    if path.exists() {
        if path.join(".git").exists() {
            return Err(unsafe_path("Existing Git repositories are out of scope for v0.1."));
        }
        let is_empty = fs::read_dir(path)
            .map_err(|error| AppError::io("Could not inspect target path.", error))?
            .next()
            .is_none();
        if !is_empty {
            return Err(AppError::new(
                "BD002",
                "Target path already exists and is not empty.",
                "`branchdojo new` refuses to overwrite existing files.",
                "Choose a new empty path.",
            ));
        }
    }
    Ok(())
}

pub fn ensure_branchdojo_workspace(path: &Path) -> AppResult<BranchDojoState> {
    ensure_path_not_unsafe(path)?;
    let state = read_state(path)?;
    if exercises::get(&state.exercise).is_none() {
        return Err(AppError::new(
            "BD006",
            format!("Unsupported exercise: {}.", state.exercise),
            "The workspace references an unknown exercise ID.",
            "Run `branchdojo list`.",
        ));
    }
    Ok(state)
}

pub fn ensure_reset_allowed(path: &Path) -> AppResult<BranchDojoState> {
    ensure_path_not_unsafe(path).map_err(|error| {
        AppError::new(
            "BD008",
            "Reset refused because this is not a valid BranchDojo workspace.",
            error.to_string(),
            "Create a new exercise or use a valid BranchDojo workspace.",
        )
    })?;
    if !path.exists() {
        return Err(reset_refused("The target path does not exist."));
    }
    ensure_branchdojo_workspace(path).map_err(|error| {
        AppError::new(
            "BD008",
            "Reset refused because this is not a valid BranchDojo workspace.",
            error.to_string(),
            "Create a new exercise or use a valid BranchDojo workspace.",
        )
    })
}

pub fn ensure_path_not_unsafe(path: &Path) -> AppResult<()> {
    if path.as_os_str().is_empty() {
        return Err(unsafe_path("The path is empty."));
    }
    if path.components().any(|component| component == Component::ParentDir) {
        return Err(unsafe_path("Parent directory references are not allowed."));
    }
    if path.parent().is_none() && path.has_root() {
        return Err(unsafe_path("Filesystem roots are not valid exercise paths."));
    }
    if matches!(path.file_name().and_then(|name| name.to_str()), Some("") | None) {
        return Err(unsafe_path("The path must include a directory name."));
    }

    let absolute = absolutize(path);
    if let Ok(home) = env::var("HOME") {
        let home_path = PathBuf::from(home);
        if absolute == home_path {
            return Err(unsafe_path("The home directory is not a valid exercise path."));
        }
    }

    let absolute_text = absolute.to_string_lossy().to_lowercase();
    let system_prefixes = [
        "/bin",
        "/etc",
        "/library",
        "/private",
        "/sbin",
        "/system",
        "/usr",
        "/var",
        "c:\\windows",
        "c:\\program files",
    ];
    if system_prefixes.iter().any(|prefix| {
        let slash_prefix = format!("{prefix}/");
        let backslash_prefix = format!("{prefix}\\");
        absolute_text == *prefix
            || absolute_text.starts_with(&slash_prefix)
            || absolute_text.starts_with(&backslash_prefix)
    }) {
        return Err(unsafe_path("System directories are not valid exercise paths."));
    }

    Ok(())
}

fn absolutize(path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }
    env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(path)
}

fn unsafe_path(cause: impl Into<String>) -> AppError {
    AppError::new(
        "BD003",
        "Unsafe path refused.",
        cause,
        "Use a dedicated local exercise path such as `./dojo-conflict-basic`.",
    )
}

fn reset_refused(cause: impl Into<String>) -> AppError {
    AppError::new(
        "BD008",
        "Reset refused because this is not a valid BranchDojo workspace.",
        cause,
        "Create a new exercise or use a valid BranchDojo workspace.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent_directory_target_is_unsafe() {
        assert!(ensure_path_not_unsafe(Path::new("../dojo")).is_err());
    }

    #[test]
    fn relative_named_directory_is_allowed() {
        assert!(ensure_path_not_unsafe(Path::new("dojo-conflict-basic")).is_ok());
    }
}
