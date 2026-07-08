# Test Plan

BranchDojo is tested with a mix of pure unit tests and integration tests that create real temporary Git repositories.

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
- Metadata completeness and metadata-driven listing output.
- Exercise creation for all implemented exercises.
- Generated `.branchdojo.json` and `README.branchdojo.md`.
- Local Git identity in generated repositories.
- Human and JSON check output.
- Safe reset refusal for non-BranchDojo directories.
- Non-empty target directory refusal.
- Invalid and missing workspace metadata errors.
- Dirty working tree and conflict marker failures.
- Simulated valid solutions for implemented exercises.
- Warning behavior for valid but unusual workflow shapes.

## v0.2 Release Smoke Checks

Before tagging v0.2.0, install locally and run:

```bash
cargo install --path . --locked --force
branchdojo --help
branchdojo list
branchdojo new conflict-basic --path ./branchdojo-smoke-conflict
branchdojo check --path ./branchdojo-smoke-conflict
branchdojo check --path ./branchdojo-smoke-conflict --json
branchdojo hint --path ./branchdojo-smoke-conflict
branchdojo reset --path ./branchdojo-smoke-conflict
```

Repeat `new`, `check`, `check --json`, `hint`, and `reset` for every implemented exercise, then remove all smoke workspaces.

## Planned v0.3 Coverage

v0.3 planning adds these test areas:

- State-aware hint tests for dirty working trees, wrong branches, conflict markers, active Git operations, and detached HEAD.
- Report output tests for explicit Markdown file creation.
- Report file safety tests for unsafe paths, existing files, and `.git` paths.
- Additional metadata usage tests for future hints and reports.
- No-regression integration tests for all six released exercises.
- Output compatibility tests for existing human and JSON check output.
