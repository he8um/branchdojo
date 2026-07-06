# Dependency Policy

BranchDojo should remain small and easy to audit.

## Runtime Dependencies

Allowed for v0.1:

- `clap` for CLI parsing.
- `serde` for serialization.
- `serde_json` for JSON output and state file.
- `anyhow` or `thiserror` for error handling.
- `time` or `chrono` for timestamps.

## Dev/Test Dependencies

Allowed for v0.1:

- `tempfile` for temporary repositories.
- `assert_cmd` for CLI integration tests.
- `predicates` for output assertions.

## Not Allowed in v0.1

- TUI libraries.
- GUI frameworks.
- Network clients.
- GitHub API clients.
- Config parser libraries for custom exercises.
- Shell command helper libraries that hide execution details.

## Review Rule

Any new dependency must answer:

1. Why is it necessary?
2. Can standard library solve it?
3. Does it increase security risk?
4. Does it complicate cross-platform behavior?
