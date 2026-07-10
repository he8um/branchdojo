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

## v0.3 Scope

BranchDojo v0.3 improves feedback and practice artifacts while preserving the v0.2 exercise catalog.

- Shared exercise metadata.
- Metadata-backed `branchdojo list` output.
- Progress-aware hints based on observable repository state.
- Static/general hint fallback.
- Optional Markdown check reports.
- Report path safety.

## v0.3 Commands

v0.3 preserves the v0.2 commands and adds:

- `branchdojo check --path <path> --report <file>`
- `branchdojo check --path <path> --json --report <file>`

## Out of Scope for v0.3

- JSON report files.
- HTML or PDF reports.
- Report templates.
- `--force` report overwrite.
- New exercises.
- External exercise packs.
- Classroom dashboards.
- GitHub integration.
- TUI or GUI.
- Release automation.

## v0.4 Scope In Development

BranchDojo v0.4 is planned around advanced built-in Git practice and reusable exercise foundations.

- Implemented on main as unreleased v0.4 work: `interactive-rebase-basic`.
- Remaining advanced exercise candidates: `bisect-basic`, `merge-vs-rebase`, and `tag-release-fix`.
- Documentation-only learning paths for beginner, intermediate, and advanced practice.
- Exercise metadata refinements for advanced difficulty and clearer grouping.
- Validator, hint, and report compatibility for new built-in exercises.
- Research for future exercise-pack and custom-definition foundations.

v0.4 should implement only the advanced exercises that remain simple, deterministic, and safe under BranchDojo's final-state validation model.

## Out of Scope for v0.4 Planning

- Implementing v0.4 features in the planning baseline.
- External exercise pack loading.
- Custom exercise definitions.
- YAML, TOML, or JSON exercise parsers.
- Classroom dashboards.
- GitHub integration.
- Remote repository workflows.
- Telemetry or analytics.
- GUI or TUI.
- Report templates.
- HTML or PDF reports.
- Command-history or shell-history tracking.

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
