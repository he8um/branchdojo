# Exercise Design: stash-switch

## Purpose

`stash-switch` teaches the user to save incomplete local work before switching branches, complete urgent work elsewhere, then restore and commit the original work.

## Starting Repository State

BranchDojo should create:

- `main` branch.
- `feature/settings-copy` branch.
- `hotfix/banner-copy` branch.
- A committed baseline file for application copy.
- Uncommitted work on `feature/settings-copy`.
- A hotfix branch that requires a small committed change.

The repository should start on `feature/settings-copy` with local uncommitted changes that would make branch switching unsafe without stashing or otherwise preserving the work.

## User Task

Preserve the local feature work, switch to `hotfix/banner-copy`, complete and commit the urgent banner fix, then return to `feature/settings-copy` and commit the original feature work.

## Expected Final State

- `feature/settings-copy` exists.
- `hotfix/banner-copy` exists.
- Working tree is clean.
- No merge, rebase, cherry-pick, or revert operation is active.
- The feature branch contains the expected settings copy change.
- The hotfix branch contains the expected banner fix.
- The feature change is not committed only on the hotfix branch.
- The hotfix change is not committed only on the feature branch.

## Required Validation Checks

- `metadata_exists`.
- `git_directory_exists`.
- `working_tree_clean`.
- `no_active_git_operation`.
- `branch_exists:feature/settings-copy`.
- `branch_exists:hotfix/banner-copy`.
- `file_contains:settings_copy_on_feature`.
- `file_contains:banner_fix_on_hotfix`.
- `feature_excludes_hotfix_only_work` if the scenario needs branch separation.
- `hotfix_excludes_feature_only_work` if the scenario needs branch separation.

## Warning Conditions

- Final branch content is correct, but the stash workflow is not visible from history or reflog signals.
- Final branch content is correct, but extra unrelated commits exist without changing required files.

Warnings must not be used when required content is missing, the working tree is dirty, or branches are missing.

## Hints

1. Start with `git status`.
2. Save your local changes before switching branches.
3. Switch to the hotfix branch and commit the urgent fix.
4. Return to the feature branch.
5. Restore the saved local work.
6. Commit the feature work.
7. Run `branchdojo check --path .`.

## Edge Cases

- User commits feature work before switching instead of stashing.
- User applies the feature work on the wrong branch.
- User leaves the stash unapplied.
- User leaves uncommitted changes after applying the stash.
- User drops expected work while cleaning up.
- User deletes `.branchdojo.json`.

## Test Cases

- New exercise starts on `feature/settings-copy` with uncommitted work.
- Initial check fails before solving.
- Valid stash-based solution passes.
- Valid commit-before-switch solution returns `WARNING` if accepted.
- Dirty working tree fails.
- Missing hotfix branch fails.
- Feature work only on hotfix branch fails.
- JSON check output includes expected status and checks.
