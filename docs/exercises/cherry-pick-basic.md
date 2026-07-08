# Exercise Design: cherry-pick-basic

## Purpose

`cherry-pick-basic` teaches the user to move one specific commit from a source branch onto the intended target branch without bringing unrelated commits.

## Starting Repository State

BranchDojo should create:

- `main` branch.
- `release/1.0` branch.
- `feature/payment-copy` branch.
- A source branch containing one useful fix commit and one unrelated follow-up commit.
- A target branch that needs only the useful fix.

The repository should start on `release/1.0`.

## User Task

Apply the specific useful commit from `feature/payment-copy` onto `release/1.0` without bringing unrelated work.

## Expected Final State

- User is on `release/1.0`.
- Working tree is clean.
- No merge, rebase, cherry-pick, or revert operation is active.
- `release/1.0` contains the expected useful fix content.
- `release/1.0` does not contain unrelated feature content.
- `feature/payment-copy` still exists.
- The useful source commit remains reachable from `feature/payment-copy`.

## Required Validation Checks

- `metadata_exists`.
- `git_directory_exists`.
- `working_tree_clean`.
- `no_active_git_operation`.
- `branch_exists:release/1.0`.
- `branch_exists:feature/payment-copy`.
- `file_contains:useful_fix_on_release`.
- `file_not_contains:unrelated_feature_content_on_release`.
- `source_commit_still_exists`.

## Warning Conditions

- Final content is correct, but the target branch does not contain a cherry-pick-style commit signal.
- Final content is correct through a patch-based fix commit.

Warnings must not be used when unrelated feature content is present on the release branch.

## Hints

1. Inspect the graph with `git log --oneline --decorate --graph --all`.
2. Identify the single commit that contains the useful fix.
3. Make sure you are on `release/1.0`.
4. Apply only that commit.
5. Confirm unrelated feature content is absent.
6. Run `branchdojo check --path .`.

## Edge Cases

- User merges the entire feature branch.
- User cherry-picks the wrong commit.
- User cherry-picks both useful and unrelated commits.
- User leaves an active cherry-pick operation unresolved.
- User recreates content manually with a valid final state.
- User deletes the source branch.

## Test Cases

- New exercise creates source and target branches.
- Initial check fails before solving.
- Valid cherry-pick solution passes.
- Patch-based solution returns `WARNING` if accepted.
- Merging unrelated content fails.
- Active cherry-pick operation fails.
- Missing source branch fails.
- JSON check output includes expected status and checks.
