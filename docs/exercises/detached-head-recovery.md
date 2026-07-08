# Exercise Design: detached-head-recovery

## Purpose

`detached-head-recovery` teaches the user to recover useful work committed on detached HEAD by making it reachable from a named branch.

## Starting Repository State

BranchDojo should create:

- `main` branch.
- `feature/recovery-target` branch.
- A detached HEAD checkout at an earlier commit.
- One useful commit made while detached.

The repository should start in detached HEAD state after the useful detached commit has been created.

## User Task

Recover the detached work by making it reachable from `feature/recovery-target`, then leave the repository on that branch with a clean working tree.

## Expected Final State

- User is on `feature/recovery-target`.
- Working tree is clean.
- No merge, rebase, cherry-pick, or revert operation is active.
- The useful detached work is reachable from `feature/recovery-target`.
- The expected recovered file content exists on `feature/recovery-target`.
- `main` remains available.
- `.branchdojo.json` remains intact.

## Required Validation Checks

- `metadata_exists`.
- `git_directory_exists`.
- `current_branch_feature_recovery_target`.
- `working_tree_clean`.
- `no_active_git_operation`.
- `branch_exists:main`.
- `branch_exists:feature/recovery-target`.
- `file_contains:recovered_detached_work`.
- `detached_work_reachable_from_target_branch`.

## Warning Conditions

- Final content is correct, but the detached commit itself is not preserved and an equivalent recovery commit was created.
- Final content is correct, but extra unrelated commits exist on the target branch.

Warnings must not be used when the recovered work is not reachable from the target branch.

## Hints

1. Run `git status` and notice detached HEAD state.
2. Inspect recent commits with `git log --oneline --decorate --graph --all`.
3. Create or update the intended branch so it includes the detached work.
4. Switch to `feature/recovery-target`.
5. Confirm the recovered content exists.
6. Run `branchdojo check --path .`.

## Edge Cases

- User leaves the repository in detached HEAD state.
- User creates a differently named branch but does not update the intended branch.
- User recovers content manually but loses the original detached commit.
- User resets away the detached work.
- User leaves an active cherry-pick or merge operation.
- User deletes `main`.

## Test Cases

- New exercise starts in detached HEAD state with useful work present.
- Initial check fails before solving.
- Branching from detached HEAD and updating the target branch passes.
- Equivalent manual recovery returns `WARNING` if accepted.
- Detached HEAD final state fails.
- Missing target branch fails.
- Missing recovered content fails.
- JSON check output includes expected status and checks.
