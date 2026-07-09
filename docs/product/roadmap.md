# Roadmap

## v0.1: Local Beginner MVP

Released:

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

Released:

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
- Progress-aware hints.

## v0.3: Better Feedback and Practice Artifacts

Release-candidate ready:

- Progress-aware hints based on current repository state. Implemented.
- Optional Markdown check reports written only when requested. Implemented.
- Richer exercise metadata for listings, hints, reports, and docs. Listing metadata foundation is implemented.
- README and onboarding polish.

Non-goals:

- GUI or TUI.
- GitHub integration or remote repository workflows.
- External exercise pack loading.
- Command-history or shell-history tracking.
- Analytics or telemetry.
- Release automation.

Implementation order:

1. Exercise metadata improvements. Foundation implemented.
2. Progress-aware hints. Implemented.
3. Check report output. Implemented.
4. Documentation and release polish.

Release criteria:

- All six released exercises still pass regression tests.
- State-aware hints are deterministic and tested.
- Report writing is explicit, safe, and tested.
- Metadata shown in output and docs matches exercise definitions.
- `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` pass.

## v0.4: Advanced Practice and Packs

Future ideas:

- `interactive-rebase-basic`.
- `bisect-basic`.
- `merge-vs-rebase`.
- `tag-release-fix`.
- Classroom mode foundations.
- Exercise packs and custom definitions.
- Exercise metadata versioning.

## v0.5: Distribution and Automation

- GitHub Action mode.
- Prebuilt binaries.
- Homebrew installation.
- Release automation.
- Signed release artifacts if needed.
