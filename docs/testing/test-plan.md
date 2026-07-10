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
- Optional Markdown check report output.
- Progress-aware hint output and static general hint fallback.
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

## v0.3 Coverage

Implemented v0.3 coverage:

- State-aware hint tests for dirty working trees, wrong branches, conflict markers, active Git operations, and detached HEAD.
- Hint refusal for missing workspace metadata.
- Hint command regression coverage for all six released exercises.
- Catalog drift prevention between exercise definitions and metadata.
- Markdown report creation and content.
- Report file safety for existing files, directories, missing parents, and `.git` paths.
- `--json --report` compatibility with parseable JSON stdout.

Future coverage:

- Additional metadata usage tests for future hints and reports.
- No-regression integration tests for all six released exercises.
- Output compatibility tests for existing human and JSON check output.

## v0.4 Planned Coverage

Implemented v0.4 coverage:

- `interactive-rebase-basic` setup tests.
- Starting-state failure and JSON shape tests for `interactive-rebase-basic`.
- Clean-history passing solution test.
- Correct-content but messy-history warning test.
- Failure tests for wrong branch, dirty working tree, remaining debug content, and missing profile content.
- Progress-aware hint test.
- Markdown report metadata test.
- Reset test.
- Metadata/list tests for `advanced` difficulty and `interactive-rebase-basic`.
- `tag-release-fix` setup tests.
- Starting-state failure and JSON shape tests for `tag-release-fix`.
- Annotated-tag passing solution test.
- Lightweight-tag warning test.
- Failure tests for missing tag, wrong tag target, wrong branch, dirty working tree, and missing fixed content.
- Progress-aware hint test.
- Markdown report metadata test.
- Reset test.
- Metadata/list tests for `tag-release-fix`.
- `merge-vs-rebase` setup tests.
- Starting-state failure and JSON shape tests for `merge-vs-rebase`.
- Linear-history passing solution test.
- Merge-commit warning solution test.
- Failure tests for wrong branch, dirty working tree, missing pricing content, missing checkout content, and missing feature branch.
- Progress-aware hint test.
- Markdown report metadata test.
- Reset test.
- Metadata/list tests for `merge-vs-rebase`.

Planned v0.4 test areas for remaining candidates:

- No-regression tests for all six released exercises.
- Deterministic bisect test strategy if `bisect-basic` remains in scope.

`bisect-basic` should only be implemented if the failing/passing behavior can be represented by deterministic local files and explicit Git commands without shell scripts, network access, or external tooling.
