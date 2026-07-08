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

## Available Exercises

| Exercise | Difficulty | Skill focus |
|---|---:|---|
| `conflict-basic` | Beginner | Merge conflicts, branch awareness, clean working tree |
| `revert-mistake` | Beginner | Safe history-preserving recovery from a bad commit |
| `wrong-branch-commit` | Beginner | Moving accidental work to the correct branch |
| `stash-switch` | Intermediate | Preserving local work before switching branches |
| `cherry-pick-basic` | Intermediate | Applying one specific commit without unrelated changes |

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

## Safety Model

BranchDojo only works with disposable repositories it creates. It refuses to reset or check unknown directories without a valid `.branchdojo.json` state file.

Safety rules:

- Never modify a non-BranchDojo repository.
- Never reset a path without a valid `.branchdojo.json` file.
- Refuse to create an exercise inside a non-empty directory.
- Refuse unsafe paths such as home, root, system, or parent directories.
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
- v0.2 in progress: stash switching and basic cherry-pick implemented; detached HEAD recovery and validation refinements planned.
- v0.3: rebase, squash, rename conflict, generated reports.
- v0.4: bisect, classroom mode, exercise packs, custom definitions.
- v0.5: GitHub Action mode, binaries, Homebrew, release automation.

## License

MIT.
