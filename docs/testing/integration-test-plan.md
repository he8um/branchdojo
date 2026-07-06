# Integration Test Plan

## CLI Tests

- `branchdojo list` prints all MVP exercises.
- Unsupported command returns useful error.
- Unsupported exercise returns `BD006`.

## New Exercise Tests

### conflict-basic

- Creates workspace.
- Creates `.git`.
- Writes `.branchdojo.json`.
- Writes `README.branchdojo.md`.
- Leaves current branch on `main`.
- Creates `feature/landing-copy`.

### revert-mistake

- Creates bad commit.
- Bad content exists before solving.
- Metadata references `config.txt`.

### wrong-branch-commit

- Creates `feature/profile-page`.
- Accidental content exists on `main` before solving.

## Check Tests

For each exercise:

- Check fails before solving.
- Simulated solution passes or warns as expected.
- Dirty working tree fails.
- Missing expected file fails.

## Reset Tests

- Reset refuses directory without metadata.
- Reset refuses invalid metadata.
- Reset recreates valid exercise.

## JSON Tests

- `check --json` returns valid JSON.
- JSON includes exercise, status, score, total, checks, next_steps.
- Check IDs are stable.
