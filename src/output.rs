use crate::error::{AppError, AppResult};
use crate::result::{CheckStatus, OverallStatus, ValidationResult};

pub fn print_human_result(result: &ValidationResult) {
    println!("BranchDojo Result\n");
    println!("Exercise: {}", result.exercise);
    println!("Status: {}", result.status.as_human());
    println!("Score: {}/{}", result.score, result.total);
    println!("\nChecks:");
    for check in &result.checks {
        let icon = match check.status {
            CheckStatus::Passed => "✅",
            CheckStatus::Warning => "⚠️",
            CheckStatus::Failed => "❌",
        };
        match &check.message {
            Some(message) => println!("{icon} {} - {message}", check.label),
            None => println!("{icon} {}", check.label),
        }
    }
    if result.status != OverallStatus::Passed && !result.next_steps.is_empty() {
        println!("\nNext:");
        for step in &result.next_steps {
            println!("{step}");
        }
    }
}

pub fn print_json_result(result: &ValidationResult) -> AppResult<()> {
    println!("{}", json_result(result)?);
    Ok(())
}

pub fn json_result(result: &ValidationResult) -> AppResult<String> {
    serde_json::to_string_pretty(result).map_err(|error| {
        AppError::new(
            "BD007",
            "Could not serialize validation result.",
            error.to_string(),
            "Run the command again. If this continues, recreate the exercise workspace.",
        )
    })
}
