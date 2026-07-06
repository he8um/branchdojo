#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckStatus {
    Passed,
    Failed,
    Warning,
}

impl CheckStatus {
    pub fn as_json(&self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Warning => "warning",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Severity {
    Required,
    Warning,
}

impl Severity {
    pub fn as_json(&self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Warning => "warning",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OverallStatus {
    Passed,
    Warning,
    Failed,
}

impl OverallStatus {
    pub fn as_human(&self) -> &'static str {
        match self {
            Self::Passed => "PASSED",
            Self::Warning => "WARNING",
            Self::Failed => "FAILED",
        }
    }

    pub fn as_json(&self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Warning => "warning",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckResult {
    pub id: String,
    pub label: String,
    pub status: CheckStatus,
    pub severity: Severity,
    pub message: Option<String>,
}

impl CheckResult {
    pub fn required(id: impl Into<String>, label: impl Into<String>, passed: bool) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            status: if passed {
                CheckStatus::Passed
            } else {
                CheckStatus::Failed
            },
            severity: Severity::Required,
            message: None,
        }
    }

    pub fn warning(id: impl Into<String>, label: impl Into<String>, warned: bool) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            status: if warned {
                CheckStatus::Warning
            } else {
                CheckStatus::Passed
            },
            severity: Severity::Warning,
            message: None,
        }
    }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationResult {
    pub exercise: String,
    pub status: OverallStatus,
    pub score: usize,
    pub total: usize,
    pub checks: Vec<CheckResult>,
    pub next_steps: Vec<String>,
}

impl ValidationResult {
    pub fn new(
        exercise: impl Into<String>,
        checks: Vec<CheckResult>,
        next_steps: Vec<String>,
    ) -> Self {
        let total = checks
            .iter()
            .filter(|check| check.severity == Severity::Required)
            .count();
        let score = checks
            .iter()
            .filter(|check| {
                check.severity == Severity::Required && check.status == CheckStatus::Passed
            })
            .count();
        let has_required_failure = checks.iter().any(|check| {
            check.severity == Severity::Required && check.status == CheckStatus::Failed
        });
        let has_warning = checks.iter().any(|check| {
            check.severity == Severity::Warning && check.status == CheckStatus::Warning
        });
        let status = if has_required_failure {
            OverallStatus::Failed
        } else if has_warning {
            OverallStatus::Warning
        } else {
            OverallStatus::Passed
        };

        Self {
            exercise: exercise.into(),
            status,
            score,
            total,
            checks,
            next_steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_required_check_controls_overall_status() {
        let result = ValidationResult::new(
            "sample",
            vec![
                CheckResult::required("a", "A", true),
                CheckResult::required("b", "B", false),
                CheckResult::warning("c", "C", true),
            ],
            vec![],
        );

        assert_eq!(result.status, OverallStatus::Failed);
        assert_eq!(result.score, 1);
        assert_eq!(result.total, 2);
    }

    #[test]
    fn warning_status_requires_all_required_checks_to_pass() {
        let result = ValidationResult::new(
            "sample",
            vec![
                CheckResult::required("a", "A", true),
                CheckResult::warning("c", "C", true),
            ],
            vec![],
        );

        assert_eq!(result.status, OverallStatus::Warning);
    }
}
