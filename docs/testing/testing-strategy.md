# Testing Strategy

BranchDojo requires strong tests because it creates repositories, changes branches, and resets files.

## Test Types

### Unit Tests

Focus on pure logic:

- State parsing.
- State validation.
- Safety checks.
- Result aggregation.
- Line ending normalization.
- Conflict marker detection.
- JSON serialization.

### Integration Tests

Use real temporary Git repositories:

- Exercise creation.
- Git branch setup.
- Validation before solving.
- Simulated valid solution.
- Reset behavior.
- JSON output.
- Error behavior.

### Manual QA

Run exercises manually before release.

## Required Commands

CI should run:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Test Isolation

All integration tests must use `tempfile`. No test may run against the developer's current repository or home directory.

## Git Dependency

Integration tests need Git. If Git is unavailable, the test suite should fail clearly or skip Git-dependent tests with an explicit message.
