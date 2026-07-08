# Roadmap

## v0.1: Local Beginner MVP

- Rust CLI.
- `conflict-basic`.
- `revert-mistake`.
- `wrong-branch-commit`.
- `.branchdojo.json` state file.
- Final-state validation.
- Human output.
- JSON output.
- Static hints.
- Safe reset.
- Unit and integration tests.
- GitHub Actions CI.

## v0.2: Intermediate Recovery Exercises

Planned scope:

- `stash-switch`: implemented; practice saving local work before switching branches.
- `cherry-pick-basic`: implemented; practice moving one specific commit onto the intended branch.
- `detached-head-recovery`: implemented; practice making detached HEAD work reachable from a branch.

Expected validation approach:

- Preserve final-state validation instead of command sequence validation.
- Require clean working trees, intact metadata, expected branches, and expected file content.
- Use `WARNING` for valid final states with unusual history shape when the exercise allows it.
- Keep validators deterministic and independent of shell history.

Out of scope for v0.2:

- Network features.
- GitHub API integration.
- TUI or GUI.
- Custom external exercise packs.
- Generated result reports.
- Release automation.
- Progress-aware hints until the new exercise validation rules are stable.

## v0.3: Better Feedback

- `rebase-linear-history`.
- `squash-before-pr`.
- `rename-conflict`.
- Improved scoring.
- Generated result reports.
- Optional Markdown result output.

## v0.4: Packs and Classroom Foundations

- `bisect-bug-hunt`.
- Classroom mode foundations.
- Exercise packs.
- Custom exercise definitions.
- Exercise metadata versioning.

## v0.5: Distribution and Automation

- GitHub Action mode.
- Prebuilt binaries.
- Homebrew installation.
- Release automation.
- Signed release artifacts if needed.
