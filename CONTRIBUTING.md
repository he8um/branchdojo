# Contributing to BranchDojo

Thank you for considering a contribution. BranchDojo is a small deterministic Git kata tool, so contributions should keep the project safe, simple, offline, and easy to test.

## Principles

1. Exercises must be deterministic.
2. Exercises must be disposable and local.
3. BranchDojo must not modify existing user repositories.
4. Validators should check final repository state, not exact command history.
5. All filesystem operations must be guarded.
6. Git must be invoked through explicit command arguments, not shell scripts.
7. Every exercise needs tests.
8. User-facing messages must be practical and beginner-friendly.

## Development Setup

```bash
git clone <repo-url>
cd branchdojo
cargo test
```

## Quality Checks

Before opening a pull request, run:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Adding an Exercise

A new exercise must include:

- Exercise metadata.
- Setup implementation.
- Generated `README.branchdojo.md` content.
- Required validation checks.
- Warning validation checks where relevant.
- Static hints.
- Unit tests for validators.
- Integration test for generated repository setup and a simulated solution.
- Documentation in `docs/exercises/`.

## Pull Request Checklist

- The change is deterministic and offline.
- No global Git configuration is modified.
- No real user repository is modified by tests.
- All tests use temporary directories.
- The docs are updated.
- Error messages include next steps.
- Public files contain no private implementation notes.
