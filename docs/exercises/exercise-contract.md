# Exercise Contract

Every BranchDojo exercise must follow a common contract.

## Required Metadata

| Field | Description |
|---|---|
| `id` | Stable exercise ID, kebab-case. |
| `title` | Human-readable title. |
| `difficulty` | Beginner, intermediate, or advanced. |
| `estimated_time` | Expected completion time. |
| `skills` | Skills practiced. |
| `description` | Short explanation. |
| `goal` | Final learning goal. |
| `setup_summary` | What BranchDojo creates. |
| `user_task` | What the user must do. |
| `validation_rules` | Required and warning checks. |
| `hints` | Static hints for v0.1. |
| `success_message` | Output on success. |
| `failure_next_steps` | Practical next steps on failure. |

## Required Setup Behavior

An exercise setup must:

- Initialize a repository.
- Set local Git identity.
- Create deterministic files.
- Create deterministic branches and commits.
- Write `.branchdojo.json`.
- Write `README.branchdojo.md`.
- Leave the repository in the intended starting state.

## Required Validation Behavior

An exercise validator must:

- Read state file.
- Run common checks.
- Run exercise-specific checks.
- Return structured results.
- Prefer final-state validation.
- Use warnings for non-ideal but valid solutions.

## Exercise ID Rules

- Lowercase.
- Kebab-case.
- Stable once released.
- Should describe the workflow scenario.

Examples:

- `conflict-basic`
- `revert-mistake`
- `wrong-branch-commit`
