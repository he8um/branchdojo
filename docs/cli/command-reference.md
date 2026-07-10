# Command Reference

## `branchdojo list`

Lists available exercises.

### Arguments

None.

### Output

Human-readable list of exercise IDs, difficulty, category, and estimated time.

### Example

```bash
branchdojo list
```

```text
Available exercises:

  conflict-basic               beginner      Merge conflicts      10-15 min
  revert-mistake               beginner      History repair       10-15 min
  wrong-branch-commit          beginner      Branch recovery      10-15 min
  stash-switch                 intermediate  Local changes        10-15 min
  cherry-pick-basic            intermediate  Selective history    15-20 min
  detached-head-recovery       intermediate  Recovery             15-20 min
  interactive-rebase-basic     advanced      History rewriting    20-30 min
  tag-release-fix              advanced      Release recovery     20-30 min
```

## `branchdojo new <exercise-name> --path <path>`

Creates a disposable exercise workspace.

### Required arguments

- `<exercise-name>`: supported exercise ID.
- `--path <path>`: target directory.

### Safety behavior

- Refuses unsafe paths.
- Refuses non-empty target directories.
- Refuses unsupported exercises.
- Checks Git availability.

### Example

```bash
branchdojo new conflict-basic --path ./dojo-conflict-basic
```

## `branchdojo check --path <path>`

Validates the final repository state for the exercise in the given workspace.

### Required arguments

- `--path <path>`

### Safety behavior

- Refuses missing `.branchdojo.json`.
- Refuses invalid metadata.
- Refuses unknown exercise IDs.

### Example

```bash
branchdojo check --path ./dojo-conflict-basic
```

## `branchdojo check --path <path> --json`

Same validation as `check`, but outputs structured JSON.

### Example

```bash
branchdojo check --path ./dojo-conflict-basic --json
```

## `branchdojo check --path <path> --report <file>`

Runs the same validation as `check` and writes a Markdown report to `<file>`.

### Behavior

- Report output is optional and explicit.
- Existing files are not overwritten.
- Unsafe report paths are refused.
- Parent directories must already exist.
- Reports summarize final-state validation and do not track command history.

### Example

```bash
branchdojo check --path ./dojo-conflict-basic --report ./conflict-report.md
```

## `branchdojo check --path <path> --json --report <file>`

Prints JSON to stdout and writes the Markdown report to `<file>`.

Stdout remains valid JSON and does not include report confirmation text.

## `branchdojo reset --path <path>`

Recreates the same exercise from scratch.

### Safety behavior

- Requires valid `.branchdojo.json`.
- Refuses unsafe paths.
- Refuses non-BranchDojo workspaces.
- Uses exercise ID from metadata.

### Example

```bash
branchdojo reset --path ./dojo-conflict-basic
```

## `branchdojo hint --path <path>`

Prints progress-aware hints and general static hints for the exercise.

Progress-aware hints are deterministic and read only observable repository state in a valid BranchDojo workspace. They do not inspect command history or shell history, and they do not modify the repository.

### Example

```bash
branchdojo hint --path ./dojo-conflict-basic
```

```text
Hints for conflict-basic:

Progress-aware hints:
1. No obvious blocker detected from the current repository state.

General hints:
1. Start with `git status`.
2. Make sure you are on `main`.
```
