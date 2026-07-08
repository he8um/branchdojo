# Product Scope

## v0.1 Scope

BranchDojo v0.1 includes:

- Rust CLI.
- Beginner exercise catalog.
- Disposable local repository generation.
- Final-state validation.
- Human-readable check output.
- JSON check output.
- Static hints.
- Safe reset.
- Workspace state file.
- Safety guards.
- Unit and integration tests.
- GitHub Actions CI.

## v0.1 Commands

- `branchdojo list`
- `branchdojo new <exercise-name> --path <path>`
- `branchdojo check --path <path>`
- `branchdojo check --path <path> --json`
- `branchdojo reset --path <path>`
- `branchdojo hint --path <path>`

## v0.1 Exercises

- `conflict-basic`
- `revert-mistake`
- `wrong-branch-commit`

## v0.2 Scope

BranchDojo v0.2 adds intermediate local Git workflow exercises while preserving the v0.1 CLI, safety model, and final-state validation model.

## v0.2 Exercises

- `stash-switch`
- `cherry-pick-basic`
- `detached-head-recovery`

## Out of Scope for v0.2

- Custom exercise definitions.
- Exercise packs.
- Classroom dashboard.
- GitHub API integration.
- GitHub Action mode.
- TUI or GUI.
- Remote repository operations.
- Running inside existing user repositories.
- Exact command sequence validation.
- Progress-aware hints.
- File report generation.
- Release automation.

## Out of Scope for v0.1

- Custom exercise definitions.
- Exercise packs.
- Classroom dashboard.
- GitHub API integration.
- GitHub Action mode.
- TUI or GUI.
- Remote repository operations.
- Running inside existing user repositories.
- Exact command sequence validation.
- Progress-aware hints.
- File report generation.
- Prebuilt release binaries.

## Scope Control Rule

If a feature requires tracking command history, invoking a shell, touching a user repository, or adding a network dependency, it is out of scope for v0.1.
