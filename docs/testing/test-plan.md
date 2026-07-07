# Test Plan

BranchDojo v0.1 is tested with a mix of pure unit tests and integration tests that create real temporary Git repositories.

## Required Local Checks

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Unit Coverage

- State file serialization and validation.
- Safety path checks.
- Result status and score aggregation.

## Integration Coverage

- `branchdojo list` and help output.
- Exercise creation for all v0.1 exercises.
- Generated `.branchdojo.json` and `README.branchdojo.md`.
- Local Git identity in generated repositories.
- Human and JSON check output.
- Safe reset refusal for non-BranchDojo directories.
- Non-empty target directory refusal.
- Invalid and missing workspace metadata errors.
- Dirty working tree and conflict marker failures.
- Simulated valid solutions for the v0.1 exercises.

## Release Smoke Checks

Before tagging v0.1.0, install locally and run:

```bash
cargo install --path . --locked --force
branchdojo --help
branchdojo list
branchdojo new conflict-basic --path ./branchdojo-smoke-conflict
branchdojo check --path ./branchdojo-smoke-conflict
branchdojo check --path ./branchdojo-smoke-conflict --json
branchdojo reset --path ./branchdojo-smoke-conflict
branchdojo hint --path ./branchdojo-smoke-conflict
```

Remove the smoke workspace after the check.
