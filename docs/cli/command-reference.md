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

Prints static hints for the exercise.

### Example

```bash
branchdojo hint --path ./dojo-conflict-basic
```
