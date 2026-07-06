use std::fmt::{self, Display};
use std::io;

#[derive(Debug)]
pub struct AppError {
    pub code: &'static str,
    pub message: String,
    pub cause: String,
    pub next_step: String,
}

impl AppError {
    pub fn new(
        code: &'static str,
        message: impl Into<String>,
        cause: impl Into<String>,
        next_step: impl Into<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            cause: cause.into(),
            next_step: next_step.into(),
        }
    }

    pub fn git(stderr: &str) -> Self {
        let detail = if stderr.trim().is_empty() {
            "Git did not provide details.".to_string()
        } else {
            format!("Git reported:\n{}", stderr.trim())
        };
        Self::new(
            "BD007",
            "Repository is in an unexpected state.",
            detail,
            "Run `branchdojo reset --path <path>` if the workspace metadata is valid.",
        )
    }

    pub fn io(context: impl Into<String>, error: io::Error) -> Self {
        Self::new(
            "BD007",
            context,
            error.to_string(),
            "Check the path and try again.",
        )
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}\n\nCause:\n{}\n\nNext:\n{}",
            self.code, self.message, self.cause, self.next_step
        )
    }
}

impl std::error::Error for AppError {}

pub type AppResult<T> = Result<T, AppError>;
