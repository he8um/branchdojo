# Reset Policy

`branchdojo reset` recreates an exercise from scratch. Because this may delete files inside the workspace, it must be heavily guarded.

## Reset Allowed Only If

- Path is provided.
- Path is safe.
- Path exists.
- `.branchdojo.json` exists.
- `.branchdojo.json` is valid.
- `tool == branchdojo`.
- Exercise ID is supported.

## Reset Must Refuse If

- Metadata is missing.
- Metadata is invalid.
- Path is unsafe.
- Exercise ID is unknown.
- Directory appears to be a non-BranchDojo workspace.

## Reset Behavior in v0.1

Full recreate:

1. Read metadata.
2. Store exercise ID.
3. Remove workspace contents.
4. Recreate the same exercise.
5. Write fresh metadata and README.

## Why Full Recreate?

It is simpler, deterministic, and easier to test than trying to reverse arbitrary Git state.

## User Message

After reset:

```text
Exercise reset complete.

Next:
cd <path>
cat README.branchdojo.md
```
