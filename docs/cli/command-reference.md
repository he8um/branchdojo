# Command Reference

## `branchdojo list`

Lists available exercises.

### Arguments

None.

### Output

Human-readable list of exercise IDs, difficulty, and skill focus.

### Example

```bash
branchdojo list
```

```text
Available exercises:

- cherry-pick-basic   Intermediate 10 to 15 minutes Commit selection, Cherry-pick workflow, Avoiding unrelated changes (Apply one specific bugfix commit without merging unrelated support work.)
- conflict-basic       Beginner   5 to 10 minutes  Branch awareness, Merge conflict resolution, Clean merge completion (Resolve a small merge conflict and keep both intended changes.)
- detached-head-recovery Intermediate 10 to 15 minutes Detached HEAD recovery, Branch creation, Reachability inspection (Recover useful work committed while HEAD is detached.)
- revert-mistake       Beginner   5 to 10 minutes  Reading log history, Safe recovery, Preserving history (Restore safe file content while preserving the bad commit in history.)
- stash-switch         Intermediate 10 to 15 minutes Preserving local work, Branch switching, Clean final state (Preserve uncommitted work before switching branches.)
- wrong-branch-commit  Beginner   10 to 15 minutes Branch inspection, Moving work between branches, Restoring main (Move accidental work from main to the intended feature branch.)
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
