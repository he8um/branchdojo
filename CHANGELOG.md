# Changelog

All notable changes to BranchDojo will be documented in this file.

This project follows a simple release log format and uses semantic versioning after the first public release.

## [Unreleased]

No unreleased changes.

## [0.3.0] - 2026-07-09

### Added

- Added a shared exercise metadata model for the released exercise catalog.
- Added metadata-backed `branchdojo list` output with difficulty, category, and estimated time.
- Added deterministic progress-aware hints for all released exercises.
- Added optional Markdown check reports with explicit `--report <file>` output.

### Changed

- Updated `branchdojo hint` to show repository-state guidance before the static general hints.
- Improved release documentation and v0.3 product documentation.

### Safety

- Report files are written only when explicitly requested.
- Existing report files are not overwritten.
- Unsafe report paths, directories, missing parent directories, and `.git` paths are refused.
- `--json --report` keeps stdout valid JSON.

### Deferred

- JSON report files, HTML/PDF reports, report templates, dashboards, `--force`, new exercises, external exercise packs, and release automation remain out of scope.

## [0.2.0] - 2026-07-08

### Added

- Added `cherry-pick-basic`, an intermediate exercise for applying one specific commit to a release branch.
- Added `detached-head-recovery`, an intermediate exercise for preserving work committed on detached HEAD.
- Added `stash-switch`, an intermediate exercise for preserving local work before switching branches.

### Changed

- Expanded BranchDojo from beginner-only drills into intermediate Git workflow practice.
- Exposed CLI version output for release verification.
- Preserved final-state validation for all v0.2 exercises.
- Documented warning behavior for valid final states with unusual workflow or history shape.
- Updated public exercise, validation, CLI, and testing docs for the v0.2 exercise catalog.

## [0.1.0] - 2026-07-08

### Added

- Rust CLI with `branchdojo list`, `new`, `check`, `reset`, and `hint`.
- Beginner exercises: `conflict-basic`, `revert-mistake`, and `wrong-branch-commit`.
- Disposable local Git repository generation with local-only Git identity setup.
- `.branchdojo.json` workspace state file.
- Final-state validation with `PASSED`, `WARNING`, and `FAILED` results.
- Human-readable and JSON check output.
- Static hints for each exercise.
- Safe reset guarded by BranchDojo workspace metadata.
- Safety guards for unsafe paths, non-empty target directories, and non-BranchDojo workspaces.
- Unit and integration tests.
- GitHub Actions CI.
