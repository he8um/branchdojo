# Cross-Platform Notes

BranchDojo should support macOS, Linux, and Windows.

## Path Separators

Use `Path` and `PathBuf`. Do not hardcode slash separators in logic.

## Shell Avoidance

Never rely on shell behavior. Use:

```rust
Command::new("git").args(["status", "--porcelain"])
```

Do not use:

```rust
Command::new("sh").arg("-c")
```

## Line Endings

Windows may use CRLF. Validators should normalize:

```text
\r\n -> \n
\r -> \n
```

## Git Availability

Git must be available in PATH. If not, return `BD001`.

## Tests

Integration tests should use `tempfile` and real Git repositories. Avoid assuming a fixed temporary path.

## Unicode

v0.1 exercise files should use plain ASCII content to reduce cross-platform friction.

## File Permissions

Do not rely on Unix-specific file permissions for validation in v0.1.
