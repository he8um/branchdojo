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

Released:

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

## v0.4: Advanced Git Practice and Reusable Foundations

Released:

- `interactive-rebase-basic`.
- `tag-release-fix`.
- `merge-vs-rebase`.
- `advanced` difficulty support.
- Learning-path documentation across beginner, intermediate, and advanced practice.
- Validator, hint, and report compatibility for advanced built-in exercises.
- `bisect-basic` deferred to v0.5 or later.

Frozen scope:

- Three advanced built-in exercises total.
- Learning paths remain documentation-only.
- Exercise-pack and custom-definition foundations remain future work without external loading.

Non-goals:

- No GUI or TUI.
- No GitHub integration or remote repository workflows.
- No telemetry, analytics, classroom dashboard, or remote tracking.
- No external exercise pack loading.
- No custom exercise definition parser.
- No YAML, TOML, or JSON exercise definitions.
- No report templates, PDF reports, or HTML reports.
- No command-history or shell-history tracking.
- No global Git configuration changes.

External exercise packs, dashboards, and GitHub integration remain future work.

## v0.5: Deterministic Debugging Practice

Planned:

- `bisect-basic`: advanced debugging practice for identifying a regression-introducing commit.
- Deterministic local regression fixture design.
- Small internal exercise foundation hardening where it directly supports `bisect-basic`.
- Progress-aware hints and Markdown report compatibility for the new exercise.
- No-regression coverage for all nine released exercises.

Non-goals:

- No external exercise pack loading.
- No custom exercise definition parser.
- No dashboards, telemetry, or GitHub integration.
- No GUI or TUI.
- No new report formats.
- No release automation unless separately planned.

## v0.6 and Later

- GitHub Action mode.
- Prebuilt binaries.
- Homebrew installation.
- Release automation.
- External exercise packs.
- Classroom dashboards.
- GitHub integration.
- TUI or GUI.
