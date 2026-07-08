# Changelog

All notable changes to BranchDojo will be documented in this file.

This project follows a simple release log format and uses semantic versioning after the first public release.

## [Unreleased]

### Added

- Added `cherry-pick-basic`, an intermediate exercise for applying one specific commit to a release branch.
- Added `detached-head-recovery`, an intermediate exercise for preserving work committed on detached HEAD.
- Added `stash-switch`, an intermediate exercise for preserving local work before switching branches.

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
