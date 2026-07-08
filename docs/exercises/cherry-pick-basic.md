# Exercise: cherry-pick-basic

## Summary

`cherry-pick-basic` teaches the user to apply one specific bugfix commit from a support branch onto the intended release branch without bringing unrelated support work.

## Difficulty

Intermediate.

## Estimated Time

15 to 20 minutes.

## Skills

- Commit selection.
- Cherry-pick workflow.
- Avoiding unrelated branch changes.

## Starting Repository State

BranchDojo creates:

- `main` branch.
- `support/legacy-fix` branch.
- `release/current` branch.
- `app.txt` with committed baseline checkout copy.
- `legacy.txt` on `support/legacy-fix` with legacy-only content.
- A target bugfix commit on `support/legacy-fix`.

The repository starts on `release/current`. The release branch does not contain either support branch change.

## User Task

Find the `Fix empty checkout cart` commit on `support/legacy-fix` and apply only that bugfix to `release/current`. Do not bring the unrelated legacy support change onto the release branch.

## Expected Final State

- User is on `release/current`.
- Working tree is clean.
- No merge, rebase, cherry-pick, or revert operation is active.
- `support/legacy-fix` exists.
- `release/current` exists.
- `release/current` contains `Fix: handle empty checkout cart`.
- `release/current` does not contain `Legacy support mode enabled`.
- The original `Fix empty checkout cart` commit still exists on `support/legacy-fix`.
- The bugfix content still exists on `support/legacy-fix`.
- `.git` directory is intact.
- `.branchdojo.json` is intact and valid.

## Required Validation Checks

- `metadata_exists`.
- `git_directory_exists`.
- `current_branch_release_current`.
- `working_tree_clean`.
- `no_active_git_operation`.
- `branch_exists:support/legacy-fix`.
- `branch_exists:release/current`.
- `bugfix_exists_on_release`.
- `legacy_content_absent_on_release`.
- `source_commit_still_exists`.
- `bugfix_remains_on_support`.

## Warning Conditions

- Final state is valid, but no cherry-pick-style commit subject is detected on `release/current`.
- Final state is valid, but history shape shows a broader merge.

Do not fail just because the cherry-picked commit hash differs from the original. A real cherry-pick normally creates a new commit hash.

Warnings must not be used when the bugfix is missing, unrelated legacy content is present on `release/current`, the working tree is dirty, a Git operation is active, or required branches are missing.

## Hints

1. Inspect the graph with `git log --oneline --decorate --graph --all`.
2. Find the commit named `Fix empty checkout cart` on `support/legacy-fix`.
3. Make sure you are on `release/current`.
4. Apply only that bugfix commit.
5. Confirm legacy-only content is absent from `release/current`.
6. Run `branchdojo check --path .`.

## Edge Cases

- User merges the entire support branch.
- User cherry-picks the wrong commit.
- User cherry-picks both support branch commits.
- User leaves an active cherry-pick operation unresolved.
- User recreates content manually with a valid final state.
- User deletes the source branch.
- User leaves the repository on the wrong branch.

## Test Cases

- New exercise creates source and target branches.
- Initial check fails before solving.
- Valid cherry-pick solution passes.
- Manual final-state solution returns `WARNING` if accepted.
- Merging unrelated legacy content fails.
- Active cherry-pick operation fails.
- Missing source branch fails.
- JSON check output includes expected status and checks.
- Reset recreates the starting state.
- Hint prints static guidance for the exercise.
