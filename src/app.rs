use std::fs;

use crate::cli::Command;
use crate::error::{AppError, AppResult};
use crate::exercises;
use crate::git;
use crate::output;
use crate::safety;
use crate::validators;

pub fn run(command: Command) -> AppResult<()> {
    match command {
        Command::List => {
            println!("Available exercises:\n");
            for exercise in exercises::all() {
                println!(
                    "- {:<20} {:<10} {:<16} {} ({})",
                    exercise.id,
                    exercise.difficulty,
                    exercise.estimated_time,
                    exercise.skills.join(", "),
                    exercise.description
                );
            }
            Ok(())
        }
        Command::New { exercise, path } => {
            git::ensure_git_available()?;
            safety::ensure_safe_new_path(&path)?;
            let definition = exercises::get(&exercise).ok_or_else(|| unsupported(&exercise))?;
            if !path.exists() {
                fs::create_dir_all(&path)
                    .map_err(|error| AppError::io("Could not create target path.", error))?;
            }
            (definition.setup)(&path)?;
            println!("Exercise created: {}", definition.id);
            println!("\nNext:");
            println!("cd {}", path.display());
            println!("cat README.branchdojo.md");
            Ok(())
        }
        Command::Check { path, json } => {
            let state = safety::ensure_branchdojo_workspace(&path)?;
            let result = validators::validate(&path, &state)?;
            if json {
                output::print_json_result(&result)?;
            } else {
                output::print_human_result(&result);
            }
            Ok(())
        }
        Command::Reset { path } => {
            git::ensure_git_available()?;
            let state = safety::ensure_reset_allowed(&path)?;
            let definition =
                exercises::get(&state.exercise).ok_or_else(|| unsupported(&state.exercise))?;
            fs::remove_dir_all(&path)
                .map_err(|error| AppError::io("Could not remove workspace contents.", error))?;
            fs::create_dir_all(&path)
                .map_err(|error| AppError::io("Could not recreate workspace.", error))?;
            (definition.setup)(&path)?;
            println!("Exercise reset complete.\n");
            println!("Next:");
            println!("cd {}", path.display());
            println!("cat README.branchdojo.md");
            Ok(())
        }
        Command::Hint { path } => {
            let state = safety::ensure_branchdojo_workspace(&path)?;
            let definition =
                exercises::get(&state.exercise).ok_or_else(|| unsupported(&state.exercise))?;
            println!("Hints for {}:", definition.id);
            for (index, hint) in definition.hints.iter().enumerate() {
                println!("{}. {}", index + 1, hint);
            }
            Ok(())
        }
    }
}

fn unsupported(exercise: &str) -> AppError {
    AppError::new(
        "BD006",
        format!("Unsupported exercise: {exercise}."),
        "The exercise ID is unknown.",
        "Run `branchdojo list`.",
    )
}
