# Exercise: stash-switch

## Summary

`stash-switch` teaches the user to preserve incomplete local work before switching branches, complete feature branch work, then return to a clean final state without losing the original work.

## Difficulty

Intermediate.

## Estimated Time

10 to 15 minutes.

## Skills

- Preserving local work.
- Branch switching.
- Clean final state validation.

## Starting Repository State

BranchDojo creates:

- `main` branch.
- `feature/settings-copy` branch.
- `app.txt` with committed baseline application copy.
- A committed feature branch placeholder that needs clearer settings copy.
- Uncommitted local work on `main`.

The repository starts on `main` with uncommitted local work in `app.txt`. A direct switch to `feature/settings-copy` is unsafe because the feature branch also changes `app.txt`.

## User Task

Preserve the local work on `main`, switch to `feature/settings-copy`, apply the expected settings copy, commit the feature work, return to `main`, restore the preserved local work, and commit it so the final working tree is clean.

## Expected Final State

- User is on `main`.
- `feature/settings-copy` exists.
- Working tree is clean.
- No merge, rebase, cherry-pick, or revert operation is active.
- `feature/settings-copy` contains `Settings: Save preferences with confidence.`
- `main` contains `Local note: Keep dark mode feedback for follow-up.`
- `.git` directory is intact.
- `.branchdojo.json` is intact and valid.

## Required Validation Checks

- `metadata_exists`.
- `git_directory_exists`.
- `current_branch_main`.
- `working_tree_clean`.
- `no_active_git_operation`.
- `branch_exists:feature/settings-copy`.
- `file_exists:app.txt`.
- `feature_settings_copy_exists`.
- `preserved_work_exists`.

## Warning Conditions

- Final state is valid, but history includes a WIP-style commit subject.
- Final state is valid, but feature copy appears on `main` too.

Do not fail just because there is no stash entry. A clean `git stash pop` normally removes the stash entry, so stash history is not reliable evidence.

Warnings must not be used when required content is missing, the working tree is dirty, a Git operation is active, or branches are missing.

## Hints

1. Start with `git status`.
2. Save the local work on `main` before switching branches.
3. Switch to `feature/settings-copy`.
4. Update `app.txt` with the expected settings copy and commit it.
5. Return to `main`.
6. Restore and commit the preserved local work.
7. Run `branchdojo check --path .`.

## Edge Cases

- User commits local work before switching instead of stashing.
- User applies the settings copy on `main` only.
- User leaves the stash unapplied.
- User leaves uncommitted changes after applying the stash.
- User drops expected work while cleaning up.
- User deletes `.branchdojo.json`.
- User leaves the repository on `feature/settings-copy`.

## Test Cases

- New exercise starts on `main` with uncommitted work.
- Initial check fails before solving.
- Valid stash-based solution passes.
- Valid commit-before-switch solution returns `WARNING` if accepted.
- Dirty working tree fails.
- Missing feature branch fails.
- Feature work only on `main` fails.
- JSON check output includes expected status and checks.
- Reset recreates the starting uncommitted work.
- Hint prints static guidance for the exercise.
