use std::path::PathBuf;

use crate::app;
use crate::error::AppResult;
use clap::{Parser, Subcommand};

pub enum Command {
    List,
    New {
        exercise: String,
        path: PathBuf,
    },
    Check {
        path: PathBuf,
        json: bool,
        report: Option<PathBuf>,
    },
    Reset {
        path: PathBuf,
    },
    Hint {
        path: PathBuf,
    },
}

#[derive(Parser)]
#[command(
    name = "branchdojo",
    version,
    about = "Practice real Git workflows in safe, disposable local repositories."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available exercises.
    List,
    /// Create a disposable exercise workspace.
    New {
        /// Supported exercise ID.
        #[arg(value_name = "exercise-name")]
        exercise: String,
        /// Target directory for the generated workspace.
        #[arg(long, value_name = "path")]
        path: PathBuf,
    },
    /// Validate the final repository state for an exercise workspace.
    Check {
        /// BranchDojo workspace path.
        #[arg(long, value_name = "path")]
        path: PathBuf,
        /// Emit structured JSON output.
        #[arg(long)]
        json: bool,
        /// Write a Markdown report to this file.
        #[arg(long, value_name = "file")]
        report: Option<PathBuf>,
    },
    /// Recreate the same exercise from scratch.
    Reset {
        /// BranchDojo workspace path.
        #[arg(long, value_name = "path")]
        path: PathBuf,
    },
    /// Print static hints for an exercise.
    Hint {
        /// BranchDojo workspace path.
        #[arg(long, value_name = "path")]
        path: PathBuf,
    },
}

pub fn run(args: Vec<String>) -> AppResult<()> {
    let cli = match Cli::try_parse_from(args) {
        Ok(cli) => cli,
        Err(error) => error.exit(),
    };
    app::run(cli.command.into())
}

impl From<Commands> for Command {
    fn from(command: Commands) -> Self {
        match command {
            Commands::List => Self::List,
            Commands::New { exercise, path } => Self::New { exercise, path },
            Commands::Check { path, json, report } => Self::Check { path, json, report },
            Commands::Reset { path } => Self::Reset { path },
            Commands::Hint { path } => Self::Hint { path },
        }
    }
}
