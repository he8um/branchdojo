# interactive-rebase-basic

## Status

Implemented on main as unreleased v0.4 work.

## Difficulty

Advanced.

## Estimated Time

20-30 min.

## Purpose

Practice cleaning up local branch history while preserving the intended final content.

## Starting State

A feature branch contains useful profile copy, a noisy WIP/debug commit, a debug file, a typo-fix commit, and a final polish commit.

## User Task

Clean the feature branch history and keep the intended final profile copy.

## Expected Final State

The repository is on `feature/profile-copy`, the working tree is clean, `profile.txt` contains the final profile copy, `debug.txt` is absent, and WIP/debug history is not reachable from the final branch.

## Required Validation Checks

- Metadata exists and is valid.
- Git repository exists.
- Current branch is `feature/profile-copy`.
- Working tree is clean.
- No merge, rebase, cherry-pick, or revert operation is active.
- `main` exists.
- `feature/profile-copy` exists.
- `profile.txt` exists.
- Expected final profile copy exists.
- Debug file/content is absent.

## Warning Conditions

- Final content is correct but WIP/debug commit subjects remain reachable.
- Final content is correct but branch history is broader or noisier than expected.

## Progress-Aware Hint Ideas

- If a rebase is active, suggest completing or aborting it before checking.
- If the user is on the wrong branch, mention `feature/profile-copy`.
- If the working tree is dirty, suggest resolving or committing pending changes.
- If debug content remains, mention removing `debug.txt`.
- If WIP/debug commit subjects remain reachable, mention cleaning branch history.

## Report Considerations

Reports should show whether the content checks passed separately from history-shape warnings.

## Safety Considerations

Validation must not require inspecting command history. It should inspect only repository state, branch names, file content, and commit history shape.

## Test Strategy

Implemented coverage includes setup tests, starting-state failure tests, a cleaned-history pass case, a warning case where content is correct but history remains noisy, failure cases, hints, report output, and reset.

## Risks

Interactive rebase can be hard to validate without overfitting to one command sequence.

## Deferred Questions

- Should future versions accept a wider range of clean history shapes without warning?
- Should `branchdojo list` eventually support filtering advanced exercises?
