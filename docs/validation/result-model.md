# Validation Result Model

BranchDojo validation returns one result for the final repository state.

## Overall Status

- `PASSED`: all required checks passed and no warning checks warn.
- `WARNING`: all required checks passed and at least one warning check warns.
- `FAILED`: at least one required check failed.

## Scoring

Required checks determine the score:

```text
score = passed required checks
total = total required checks
```

Warning checks do not reduce score.

## Report Use

Human output, JSON output, and Markdown check reports summarize the same validation result. Reports do not change status aggregation, scoring, warning behavior, or JSON output shape.

Reports also do not inspect command history or shell history.
