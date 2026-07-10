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

In development:

- `interactive-rebase-basic`: implemented on main as unreleased v0.4 work.
- Advanced built-in exercise candidates still in design: `bisect-basic`, `merge-vs-rebase`, and `tag-release-fix`.
- Learning-path documentation across beginner, intermediate, and advanced practice.
- Exercise metadata refinements needed for advanced difficulty and clearer grouping.
- Validator, hint, and report compatibility for new built-in exercises.

Candidate scope:

- Implement 2 or 3 advanced built-in exercises total, not necessarily all four original candidates.
- `advanced` difficulty support is implemented for the first advanced exercise.
- Keep learning paths documentation-only unless a small CLI change is separately approved.
- Research exercise-pack and custom-definition foundations without implementing external loading.

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

## v0.5: Distribution and Automation

- GitHub Action mode.
- Prebuilt binaries.
- Homebrew installation.
- Release automation.
- Signed release artifacts if needed.
