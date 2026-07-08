# Exercise: detached-head-recovery

## Summary

`detached-head-recovery` teaches the user to recover useful work committed while HEAD is detached by making that work reachable from a named branch.

## Difficulty

Intermediate.

## Estimated Time

15 to 20 minutes.

## Skills

- Detached HEAD recovery.
- Branch creation.
- Reachability inspection.

## Starting Repository State

BranchDojo creates:

- `main` branch.
- A simple project history on `main`.
- A detached HEAD checkout at an earlier commit.
- `recovered-note.txt` committed while HEAD is detached.

The repository starts in detached HEAD state after the useful detached commit has been created.

## User Task

Create or update `recovery/detached-work` so the detached work is reachable from that branch. Leave the repository on `recovery/detached-work` with a clean working tree.

## Expected Final State

- User is on `recovery/detached-work`.
- Working tree is clean.
- No merge, rebase, cherry-pick, or revert operation is active.
- `main` exists.
- `recovery/detached-work` exists.
- `recovered-note.txt` contains `Recovered detached HEAD work` on `recovery/detached-work`.
- Repository is no longer in detached HEAD state.
- The original `Add detached work note` commit is reachable from `recovery/detached-work`, or equivalent recovered content exists with a warning.
- `.git` directory is intact.
- `.branchdojo.json` is intact and valid.

## Required Validation Checks

- `metadata_exists`.
- `git_directory_exists`.
- `current_branch_recovery_detached_work`.
- `working_tree_clean`.
- `no_active_git_operation`.
- `branch_exists:main`.
- `branch_exists:recovery/detached-work`.
- `recovered_work_exists`.
- `not_detached_head`.

## Warning Conditions

- Final content is valid, but the original detached commit subject is not reachable from `recovery/detached-work`.
- Final content is valid, but recovery appears to have been recreated manually rather than preserving the original detached commit.

Do not fail just because the exact detached commit hash differs if the recovered content exists on the expected branch. Do fail if the repository is still detached or the recovered work is missing.

## Hints

1. Run `git status` and notice the detached HEAD state.
2. Inspect recent commits with `git log --oneline --decorate --graph --all`.
3. Create `recovery/detached-work` so it points at the detached work.
4. Make sure you are on `recovery/detached-work`.
5. Confirm `recovered-note.txt` contains the recovered work.
6. Run `branchdojo check --path .`.

## Edge Cases

- User leaves the repository in detached HEAD state.
- User creates a differently named branch but does not update the expected branch.
- User recovers content manually but loses the original detached commit.
- User resets away the detached work.
- User leaves an active cherry-pick or merge operation.
- User deletes `main`.

## Test Cases

- New exercise starts in detached HEAD state with useful work present.
- Initial check fails before solving.
- Branching from detached HEAD to `recovery/detached-work` passes.
- Equivalent manual recovery returns `WARNING`.
- Missing recovered work fails.
- Detached HEAD final state fails.
- Missing recovery branch fails.
- JSON check output includes expected status and checks.
- Reset recreates the detached starting state.
- Hint prints static guidance for the exercise.
