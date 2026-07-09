use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::error::{AppError, AppResult};
use crate::exercises::metadata;
use crate::result::{CheckStatus, OverallStatus, Severity, ValidationResult};
use crate::state::BranchDojoState;

pub fn render_markdown_report(
    workspace_path: &Path,
    state: &BranchDojoState,
    result: &ValidationResult,
) -> String {
    let metadata = metadata::find_metadata(&state.exercise);
    let title = metadata
        .map(|metadata| metadata.title)
        .unwrap_or("Unknown exercise");
    let difficulty = metadata
        .map(|metadata| metadata.difficulty.as_str())
        .unwrap_or("unknown");
    let category = metadata
        .map(|metadata| metadata.category)
        .unwrap_or("Unknown");
    let estimated_time = metadata
        .map(|metadata| metadata.estimated_time)
        .unwrap_or("Unknown");
    let expected_branch = metadata
        .map(|metadata| metadata.expected_final_branch)
        .unwrap_or(state.expected_branch.as_str());

    let mut report = String::new();
    report.push_str("# BranchDojo Check Report\n\n");
    report.push_str("## Summary\n\n");
    report.push_str(&format!("- Exercise: `{}`\n", state.exercise));
    report.push_str(&format!("- Title: {title}\n"));
    report.push_str(&format!("- Difficulty: {difficulty}\n"));
    report.push_str(&format!("- Category: {category}\n"));
    report.push_str(&format!("- Estimated time: {estimated_time}\n"));
    report.push_str(&format!("- Expected final branch: `{expected_branch}`\n"));
    report.push_str(&format!("- Status: {}\n", result.status.as_human()));
    report.push_str(&format!("- Score: {} / {}\n", result.score, result.total));
    report.push_str("\n## Result\n\n");
    report.push_str(
        "BranchDojo validates the final repository state, not the exact command sequence.\n",
    );
    report.push_str("\n## Checks\n\n");
    report.push_str("| Severity | Status | Check | Details |\n");
    report.push_str("|---|---|---|---|\n");
    for check in &result.checks {
        let details = check.message.as_deref().unwrap_or("");
        report.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            severity_value(&check.severity),
            check_status_value(&check.status),
            markdown_table_cell(&check.label),
            markdown_table_cell(details)
        ));
    }
    report.push_str("\n## Next Steps\n\n");
    if result.status == OverallStatus::Failed {
        report.push_str("- Resolve failed required checks.\n");
    }
    if result.status == OverallStatus::Warning {
        report.push_str(
            "- Review warning checks and decide whether to retry with a cleaner workflow.\n",
        );
    }
    for step in &result.next_steps {
        report.push_str(&format!("- {}\n", markdown_list_item(step)));
    }
    report.push_str(&format!(
        "- Run `branchdojo hint --path {}` for progress-aware guidance.\n",
        workspace_path.display()
    ));
    report.push_str(&format!(
        "- Run `branchdojo check --path {}` again.\n",
        workspace_path.display()
    ));
    report
}

pub fn write_markdown_report(report_path: &Path, content: &str) -> AppResult<()> {
    ensure_safe_report_path(report_path)?;
    fs::write(report_path, content)
        .map_err(|error| AppError::io("Could not write report file.", error))
}

fn ensure_safe_report_path(path: &Path) -> AppResult<()> {
    if path.as_os_str().is_empty()
        || path.parent().is_none() && path.has_root()
        || path
            .components()
            .any(|component| component == Component::ParentDir)
    {
        return Err(unsafe_report_path("Choose a regular report file path."));
    }

    let absolute = absolutize(path);
    if let Ok(home) = env::var("HOME") {
        if absolute == Path::new(&home) {
            return Err(unsafe_report_path(
                "The home directory is not a report file.",
            ));
        }
    }

    if absolute.components().any(|component| {
        matches!(
            component,
            Component::Normal(name) if name == ".git"
        )
    }) {
        return Err(unsafe_report_path(
            "Report files cannot be written inside .git.",
        ));
    }

    if path.exists() {
        if path.is_dir() {
            return Err(unsafe_report_path("The report path is a directory."));
        }
        return Err(AppError::new(
            "BD007",
            "Report file already exists.",
            "BranchDojo does not overwrite report files.",
            "Choose a new report path.",
        ));
    }

    let Some(parent) = path.parent() else {
        return Ok(());
    };
    if !parent.as_os_str().is_empty() && !parent.exists() {
        return Err(AppError::new(
            "BD009",
            "Report parent directory does not exist.",
            "BranchDojo does not create report parent directories.",
            "Choose an existing parent directory.",
        ));
    }
    Ok(())
}

fn markdown_table_cell(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace('|', "\\|")
}

fn markdown_list_item(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn severity_value(severity: &Severity) -> &'static str {
    match severity {
        Severity::Required => "required",
        Severity::Warning => "warning",
    }
}

fn check_status_value(status: &CheckStatus) -> &'static str {
    match status {
        CheckStatus::Passed => "passed",
        CheckStatus::Failed => "failed",
        CheckStatus::Warning => "warning",
    }
}

fn unsafe_report_path(cause: impl Into<String>) -> AppError {
    AppError::new(
        "BD008",
        "Unsafe report path.",
        cause,
        "Choose a regular file path outside .git and protected directories.",
    )
}

fn absolutize(path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }
    env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::CheckResult;

    #[test]
    fn markdown_table_cell_escapes_pipes_and_newlines() {
        assert_eq!(
            markdown_table_cell(" value | with\nnew line "),
            "value \\| with new line"
        );
    }

    #[test]
    fn renderer_includes_metadata_fields() {
        let state = BranchDojoState::new("conflict-basic", vec!["app.txt".to_string()]);
        let result = ValidationResult::new(
            "conflict-basic",
            vec![CheckResult::required("metadata", "Metadata exists", true)],
            vec!["Try again.".to_string()],
        );

        let report = render_markdown_report(Path::new("."), &state, &result);

        assert!(report.contains("# BranchDojo Check Report"));
        assert!(report.contains("- Exercise: `conflict-basic`"));
        assert!(report.contains("- Title: Resolve a basic merge conflict"));
        assert!(report.contains("- Difficulty: beginner"));
        assert!(report.contains("- Category: Merge conflicts"));
        assert!(report.contains("- Expected final branch: `main`"));
    }

    #[test]
    fn renderer_includes_warning_checks() {
        let state = BranchDojoState::new("conflict-basic", vec!["app.txt".to_string()]);
        let result = ValidationResult::new(
            "conflict-basic",
            vec![
                CheckResult::required("content", "Content exists", true),
                CheckResult::warning("shape", "History | shape\ncheck", true),
            ],
            vec![],
        );

        let report = render_markdown_report(Path::new("."), &state, &result);

        assert!(report.contains("- Status: WARNING"));
        assert!(report.contains("| warning | warning | History \\| shape check |"));
    }
}
