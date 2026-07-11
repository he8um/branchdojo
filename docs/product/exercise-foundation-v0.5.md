# Exercise Foundation Plan for v0.5

## Status

Planning only. No foundation changes are implemented yet.

## Objective

Identify the smallest internal foundation hardening that can support `bisect-basic` and future advanced exercises without introducing external exercise loading or custom definition formats.

## Current Foundation

- Built-in exercises are Rust modules.
- Metadata is hardcoded in `src/exercises/metadata.rs`.
- Git access goes through explicit `std::process::Command` arguments.
- Validators return a shared result model.
- Hints read observable repository state and stay read-only.
- Markdown reports render existing validation results.
- Tests use temporary real Git repositories.

## Pain Points Observed in v0.4

- Advanced validators repeatedly inspect file contents at refs.
- Several validators need commit-subject reachability checks.
- History-shape checks can be easy to overfit if helpers are too specific.
- Tag, merge, branch, and ancestry checks need consistent error handling.
- Future debugging exercises may need active bisect-state detection.

## Candidate Internal Helpers

- Ref content checks for files at branches, tags, or commits.
- Reachable commit subject checks.
- Merge commit checks.
- Tag object type checks.
- Branch ancestry checks.
- Known good/bad commit marker checks.
- Active bisect-state checks.
- Text normalization helpers for cross-platform fixture content.

These helpers should live only where they reduce real duplication or make validation less brittle.

## Candidate Test Helpers

- Create deterministic linear histories.
- Read file contents at refs.
- Assert branch and tag existence.
- Assert JSON output shape.
- Assert report creation and cleanup.
- Assert active Git operation state for bisect-like workflows.

Test helpers should remain local to tests unless production code has the same need.

## What Not To Build Yet

- External exercise packs.
- Custom exercise definition parser.
- User-authored exercise DSL.
- YAML, TOML, or JSON exercise definitions.
- Plugin system.
- Remote exercise registry.
- Telemetry.
- Dashboards.
- GitHub API integration.
- New report formats.

## Safety Constraints

- Never mutate non-BranchDojo repositories.
- Never reset without valid `.branchdojo.json`.
- Do not inspect shell history or command history.
- Do not require network access.
- Do not modify global Git config.
- Use explicit Git command arguments.
- Keep generated repositories disposable.

## Implementation Guidelines

- Prefer small helpers extracted from repeated validator needs.
- Keep helper names tied to observable repository facts, not command sequences.
- Add tests around helper behavior if the helper handles edge cases.
- Avoid abstractions for a single exercise unless they reduce safety risk.
- Preserve current CLI behavior and JSON schema.

## Release Criteria

- Any helper changes are covered by unit or integration tests.
- Existing validators keep current behavior.
- `bisect-basic` can validate final state deterministically.
- No external pack or custom parser surface is introduced.
- `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` pass.

## Deferred Items

- External exercise pack loading.
- Custom exercise definitions.
- User-authored exercise DSL.
- Remote exercise registry.
- Dashboards.
- Telemetry.
- GitHub integration.
- TUI or GUI.
- Release automation.
