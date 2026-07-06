# Test Fixtures

## Fixture Strategy

Prefer programmatic fixtures over static committed `.git` directories.

Why:

- Static Git directories are brittle.
- Programmatic setup mirrors real exercise creation.
- Temporary directories keep tests isolated.

## Temporary Repositories

Use `tempfile` for all integration tests:

```rust
let temp = tempfile::tempdir()?;
let path = temp.path().join("dojo");
```

## Simulated Solutions

Each exercise should have a helper that simulates a valid final state.

Examples:

- Resolve `conflict-basic` by merging and writing expected content.
- Resolve `revert-mistake` by creating a fix commit after the bad commit.
- Resolve `wrong-branch-commit` by moving content to feature branch and restoring main.

## Invalid Fixtures

Add tests for:

- Missing metadata.
- Invalid metadata.
- Missing `.git`.
- Dirty working tree.
- Missing expected files.
- Unsafe paths.

## Line Endings

Content checks should include LF and CRLF cases.
