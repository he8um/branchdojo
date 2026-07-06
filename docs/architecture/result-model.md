# Result Model

BranchDojo uses a shared result model for all validators.

## Concepts

### Check Status

- `passed`
- `failed`
- `warning`

### Severity

- `required`
- `warning`

### Overall Status

- `PASSED`
- `WARNING`
- `FAILED`

## Result Rules

```text
FAILED:
At least one required check failed.

WARNING:
All required checks passed and at least one warning condition exists.

PASSED:
All required checks passed and no warning conditions exist.
```

## Scoring

v0.1 uses simple required-check scoring:

```text
score = passed required checks
total = total required checks
```

Warnings do not reduce score. They affect overall status.

## Check Result Shape

```rust
struct CheckResult {
    id: String,
    label: String,
    status: CheckStatus,
    severity: Severity,
    message: Option<String>,
}
```

## Validation Result Shape

```rust
struct ValidationResult {
    exercise: String,
    status: OverallStatus,
    score: usize,
    total: usize,
    checks: Vec<CheckResult>,
    next_steps: Vec<String>,
}
```

## Aggregation Rule

Result aggregation should be centralized in `result.rs`, not duplicated across exercise validators.
