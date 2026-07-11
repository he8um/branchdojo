# BranchDojo

Practice real Git workflows in safe, disposable local repositories.

BranchDojo is a local Git kata tool for developers who know basic Git commands but have not practiced real workflow recovery. It creates small disposable repositories with prepared branches, commits, mistakes, and conflicts. The user solves the exercise manually with real Git commands, then BranchDojo validates the final repository state.

BranchDojo is not a visualizer, not a GUI, not a cheat sheet, and not a guided trainer. It is a deterministic practice system for Git workflow muscle memory.

## Who Is This For?

BranchDojo is designed for:

- Junior developers who know `add`, `commit`, `pull`, and `push`, but get stuck during conflicts or history mistakes.
- Vibe coder developers who can build with tooling assistance but need safer command-line Git workflow practice.
- Developers who know Git basics but have not practiced real conflict, revert, wrong-branch, and recovery scenarios.
- Bootcamp students, self-taught developers, onboarding programs, and small teams.

## Core Idea

```bash
branchdojo new conflict-basic --path ./dojo-conflict-basic
cd ./dojo-conflict-basic
cat README.branchdojo.md
# solve with real git commands
branchdojo check --path .
```

The tool generates the exercise. The user solves it. The tool validates the final repository state.

## Release State

v0.1.0, v0.2.0, and v0.3.0 are released.
v0.4.0 is released.

## Available Exercises

| Exercise | Difficulty | Category | Time |
|---|---:|---|---:|
| `conflict-basic` | Beginner | Merge conflicts | 10-15 min |
| `revert-mistake` | Beginner | History repair | 10-15 min |
| `wrong-branch-commit` | Beginner | Branch recovery | 10-15 min |
| `stash-switch` | Intermediate | Local changes | 10-15 min |
| `cherry-pick-basic` | Intermediate | Selective history | 15-20 min |
| `detached-head-recovery` | Intermediate | Recovery | 15-20 min |
| `interactive-rebase-basic` | Advanced | History rewriting | 20-30 min |
| `tag-release-fix` | Advanced | Release recovery | 20-30 min |
| `merge-vs-rebase` | Advanced | Branch integration | 20-30 min |

## Installation

Install locally from the repository:

```bash
cargo install --path .
```

## Commands

```bash
branchdojo list
branchdojo new <exercise-name> --path <path>
branchdojo check --path <path>
branchdojo check --path <path> --json
branchdojo check --path <path> --report <file>
branchdojo check --path <path> --json --report <file>
branchdojo reset --path <path>
branchdojo hint --path <path>
```

## Validation Model

BranchDojo validates the final repository state, not the exact command sequence.

This means multiple valid solutions can pass. If the final state is correct but the workflow shape is not ideal, BranchDojo returns `WARNING` instead of `FAILED`.

| Status | Meaning |
|---|---|
| `PASSED` | All required checks passed and no warnings exist. |
| `WARNING` | All required checks passed but the workflow/history shape is not ideal. |
| `FAILED` | At least one required check failed. |

## Hints

`branchdojo hint --path <path>` reads the current repository state in a valid BranchDojo workspace and prints deterministic progress-aware hints before the general exercise hints.

Hints use observable Git state such as the current branch, dirty working tree, active Git operation, conflict markers, missing branches, and expected exercise files. They do not inspect command history or shell history, and they do not modify the repository.

## Reports

`branchdojo check --path <path> --report <file>` writes an optional Markdown report for the same final-state validation result shown in the terminal.

Reports are created only when `--report` is provided. Existing files are not overwritten, unsafe paths are refused, and `branchdojo check --json --report <file>` keeps stdout as valid JSON.

## Safety Model

BranchDojo only works with disposable repositories it creates. It refuses to reset or check unknown directories without a valid `.branchdojo.json` state file.

Safety rules:

- Never modify a non-BranchDojo repository.
- Never reset a path without a valid `.branchdojo.json` file.
- Refuse to create an exercise inside a non-empty directory.
- Refuse unsafe paths such as home, root, system, or parent directories.
- Refuse unsafe report paths and refuse to overwrite existing report files.
- Do not modify global Git config.
- Do not use shell scripts for Git operations.

## Documentation

Detailed design docs are available under `docs/`:

- Product docs: `docs/product/`
- CLI docs: `docs/cli/`
- Architecture docs: `docs/architecture/`
- Exercise docs: `docs/exercises/`
- Validation docs: `docs/validation/`
- Safety docs: `docs/safety/`
- Testing docs: `docs/testing/`
- Decision records: `docs/decisions/`

## Roadmap

- v0.1: beginner exercises, state file, final-state validation, reset, hints, JSON output, CI.
- v0.2: stash switching, basic cherry-pick, detached HEAD recovery, final-state validation, warnings for valid but unusual workflow shapes.
- v0.3: progress-aware hints, optional Markdown check reports, richer exercise metadata, onboarding polish.
- v0.4: advanced built-in exercises, advanced difficulty support, and learning-path documentation. `interactive-rebase-basic`, `tag-release-fix`, and `merge-vs-rebase` are implemented.
- v0.5 planned: deterministic debugging practice through `bisect-basic` and small exercise foundation hardening.

## License

MIT.
