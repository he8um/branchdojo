# interactive-rebase-basic

## Status

Design candidate for v0.4. Not implemented.

## Difficulty

Advanced.

## Estimated Time

20-30 min.

## Purpose

Practice cleaning up local branch history while preserving the intended final content.

## Starting State

A feature branch contains several small commits, including noisy intermediate commits and at least one commit message that should be cleaned up.

## User Task

Rewrite the feature branch into a clearer history and keep the intended file changes.

## Expected Final State

The repository is on the expected feature branch, the working tree is clean, the intended content exists, and the branch history has the expected cleaned-up shape.

## Required Validation Checks

- Metadata exists and is valid.
- Git repository exists.
- Current branch is the expected feature branch.
- Working tree is clean.
- No merge, rebase, cherry-pick, or revert operation is active.
- Expected final content exists.
- Unwanted intermediate content is absent.
- Required cleaned-up commit signal exists.

## Warning Conditions

- Final content is correct but history still contains noisy fixup-style commits.
- Final content is correct but commit messages are less clear than intended.

## Progress-Aware Hint Ideas

- If a rebase is active, suggest completing or aborting it before checking.
- If the user is on the wrong branch, mention the expected feature branch.
- If the working tree is dirty, suggest resolving or committing pending changes.

## Report Considerations

Reports should show whether the content checks passed separately from history-shape warnings.

## Safety Considerations

Validation must not require inspecting command history. It should inspect only repository state, branch names, file content, and commit history shape.

## Test Strategy

Add setup tests, starting-state failure tests, a cleaned-history pass case, and a warning case where content is correct but history remains noisy.

## Risks

Interactive rebase can be hard to validate without overfitting to one command sequence.

## Deferred Questions

- Should this exercise require a specific final commit subject?
- Should squash and fixup outcomes both pass?
- Should an amend-only solution warn or fail?
