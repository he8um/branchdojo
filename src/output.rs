use crate::result::{CheckStatus, ValidationResult};
use crate::state::escape_json;

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
    if !result.next_steps.is_empty() {
        println!("\nNext:");
        for step in &result.next_steps {
            println!("{step}");
        }
    }
}

pub fn print_json_result(result: &ValidationResult) {
    println!("{}", json_result(result));
}

pub fn json_result(result: &ValidationResult) -> String {
    let checks = result
        .checks
        .iter()
        .map(|check| {
            let mut fields = vec![
                format!("      \"id\": \"{}\"", escape_json(&check.id)),
                format!("      \"label\": \"{}\"", escape_json(&check.label)),
                format!("      \"status\": \"{}\"", check.status.as_json()),
                format!("      \"severity\": \"{}\"", check.severity.as_json()),
            ];
            if let Some(message) = &check.message {
                fields.push(format!("      \"message\": \"{}\"", escape_json(message)));
            }
            format!("    {{\n{}\n    }}", fields.join(",\n"))
        })
        .collect::<Vec<_>>()
        .join(",\n");
    let next_steps = result
        .next_steps
        .iter()
        .map(|step| format!("    \"{}\"", escape_json(step)))
        .collect::<Vec<_>>()
        .join(",\n");
    format!(
        "{{\n  \"exercise\": \"{}\",\n  \"status\": \"{}\",\n  \"score\": {},\n  \"total\": {},\n  \"checks\": [\n{}\n  ],\n  \"next_steps\": [\n{}\n  ]\n}}",
        escape_json(&result.exercise),
        result.status.as_json(),
        result.score,
        result.total,
        checks,
        next_steps
    )
}
