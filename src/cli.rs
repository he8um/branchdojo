use std::path::PathBuf;

use crate::app;
use crate::error::{AppError, AppResult};

pub enum Command {
    List,
    New { exercise: String, path: PathBuf },
    Check { path: PathBuf, json: bool },
    Reset { path: PathBuf },
    Hint { path: PathBuf },
}

pub fn run(args: Vec<String>) -> AppResult<()> {
    let command = parse(args)?;
    app::run(command)
}

fn parse(args: Vec<String>) -> AppResult<Command> {
    let Some(command) = args.get(1).map(String::as_str) else {
        return Err(usage_error());
    };
    match command {
        "list" => Ok(Command::List),
        "new" => {
            let exercise = args.get(2).cloned().ok_or_else(usage_error)?;
            let path = parse_path(&args[3..])?;
            Ok(Command::New { exercise, path })
        }
        "check" => {
            let path = parse_path(&args[2..])?;
            let json = args.iter().any(|arg| arg == "--json");
            Ok(Command::Check { path, json })
        }
        "reset" => {
            let path = parse_path(&args[2..])?;
            Ok(Command::Reset { path })
        }
        "hint" => {
            let path = parse_path(&args[2..])?;
            Ok(Command::Hint { path })
        }
        _ => Err(usage_error()),
    }
}

fn parse_path(args: &[String]) -> AppResult<PathBuf> {
    args.windows(2)
        .find_map(|pair| {
            if pair[0] == "--path" {
                Some(PathBuf::from(&pair[1]))
            } else {
                None
            }
        })
        .ok_or_else(usage_error)
}

fn usage_error() -> AppError {
    AppError::new(
        "BD006",
        "Unsupported command or arguments.",
        "The command does not match the v0.1 command reference.",
        "Run `branchdojo list` or see README usage examples.",
    )
}
