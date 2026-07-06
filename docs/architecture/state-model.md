# State Model

Each BranchDojo workspace includes a `.branchdojo.json` file at the workspace root.

## Purpose

The state file prevents BranchDojo from guessing. It tells `check`, `hint`, and `reset` which exercise exists in the workspace and confirms that BranchDojo created the directory.

## Schema

```json
{
  "tool": "branchdojo",
  "schema_version": "0.1.0",
  "exercise": "conflict-basic",
  "created_at": "2026-07-07T00:00:00Z",
  "expected_branch": "main",
  "expected_files": ["app.txt"],
  "validation_policy": "final-state"
}
```

## Required Fields

| Field | Required | Meaning |
|---|---:|---|
| `tool` | yes | Must be `branchdojo`. |
| `schema_version` | yes | State schema version. |
| `exercise` | yes | Exercise ID. |
| `created_at` | yes | Creation timestamp. |
| `expected_branch` | yes | Main expected branch for validation. |
| `expected_files` | yes | Files expected by the exercise. |
| `validation_policy` | yes | Must be `final-state` in v0.1. |

## Invalid State Behavior

If the file is missing or invalid:

- `check` must refuse.
- `hint` must refuse.
- `reset` must refuse.

## Schema Version Behavior

For v0.1:

- Exact `0.1.0` is accepted.
- Unknown versions should produce a warning or error depending on compatibility.
- Simpler implementation may reject unknown versions with `BD005`.

## Security Note

The state file is a guard, not a full security boundary. It must be combined with path safety checks.
