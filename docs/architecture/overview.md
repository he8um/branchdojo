# Architecture Overview

BranchDojo is a Rust CLI with a small set of deterministic modules.

## Layers

```text
CLI layer
  -> command parsing and routing
Safety layer
  -> path checks and workspace guard
State layer
  -> .branchdojo.json read/write/validate
Git wrapper
  -> explicit git command execution
Exercise layer
  -> repository setup and generated README
Validation layer
  -> final-state checks
Result/output layer
  -> human output and JSON output
```

## Design Goals

- Keep modules small.
- Keep exercise setup deterministic.
- Avoid shell-specific behavior.
- Make safety checks explicit.
- Keep validation independent from command history.
- Keep output stable and testable.

## Main Runtime Flow: New Exercise

1. Parse CLI args.
2. Validate exercise ID.
3. Validate target path safety.
4. Check Git availability.
5. Create target directory.
6. Initialize Git repository.
7. Set local Git identity.
8. Run exercise setup.
9. Write `.branchdojo.json`.
10. Write `README.branchdojo.md`.
11. Print next steps.

## Main Runtime Flow: Check

1. Parse CLI args.
2. Validate workspace path safety.
3. Read `.branchdojo.json`.
4. Resolve exercise validator.
5. Run common checks.
6. Run exercise-specific checks.
7. Aggregate result.
8. Print human output or JSON.

## Main Runtime Flow: Reset

1. Parse CLI args.
2. Validate path safety.
3. Read `.branchdojo.json`.
4. Confirm known exercise.
5. Remove/recreate workspace.
6. Run exercise setup again.
