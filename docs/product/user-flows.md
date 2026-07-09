# User Flows

## Flow 1: Discover Exercises

1. User installs BranchDojo.
2. User runs:

```bash
branchdojo list
```

3. BranchDojo prints available exercises with difficulty, category, and estimated time.
4. User chooses an exercise.

Expected output style:

```text
Available exercises:

  conflict-basic               beginner      Merge conflicts      10-15 min
  revert-mistake               beginner      History repair       10-15 min
  wrong-branch-commit          beginner      Branch recovery      10-15 min
```

## Flow 2: Create an Exercise

1. User runs:

```bash
branchdojo new conflict-basic --path ./dojo-conflict-basic
```

2. BranchDojo checks:

- Git is available.
- Path is safe.
- Target directory is empty or absent.
- Exercise exists.

3. BranchDojo creates repository, commits, branches, metadata, and README.
4. BranchDojo prints next steps.

## Flow 3: Solve an Exercise

1. User enters generated directory.
2. User reads `README.branchdojo.md`.
3. User uses real Git commands.
4. User runs:

```bash
branchdojo check --path .
```

5. BranchDojo returns `PASSED`, `WARNING`, or `FAILED`.

## Flow 4: Ask for Hint

1. User is stuck.
2. User runs:

```bash
branchdojo hint --path .
```

3. BranchDojo reads `.branchdojo.json`.
4. BranchDojo prints progress-aware hints based on repository state.
5. BranchDojo prints general static hints for that exercise.

## Flow 5: Reset Exercise

1. User wants to start over.
2. User runs:

```bash
branchdojo reset --path .
```

3. BranchDojo validates workspace metadata.
4. BranchDojo recreates the same exercise.
5. BranchDojo refuses if metadata is missing or invalid.

## Flow 6: Machine-Readable Check

1. User or future automation runs:

```bash
branchdojo check --path . --json
```

2. BranchDojo emits structured JSON with status, score, checks, and next steps.
