# merge-vs-rebase

## Status

Design candidate for v0.4. Not implemented.

## Difficulty

Advanced.

## Estimated Time

20-30 min.

## Purpose

Practice integrating divergent branches while understanding when history shape matters.

## Starting State

`main` and a feature branch have both moved forward from a shared base. The exercise instructions identify the preferred integration style.

## User Task

Integrate the required changes and produce the requested final branch state.

## Expected Final State

The expected branch contains both sets of changes, the working tree is clean, and the history shape matches the requested integration style or produces a warning when content is valid but shape is not ideal.

## Required Validation Checks

- Metadata exists and is valid.
- Git repository exists.
- Current branch is the expected branch.
- Working tree is clean.
- No active Git operation remains.
- Required content from both branches exists.
- Conflict markers are absent.

## Warning Conditions

- Final content is correct but the alternate integration style was used.
- Extra cleanup commits exist but do not break the final content.

## Progress-Aware Hint Ideas

- If a merge or rebase is active, suggest resolving it before checking.
- If conflict markers remain, suggest resolving them and preserving both required changes.
- If the user is on the wrong branch, mention the expected final branch.

## Report Considerations

Reports should show content checks separately from merge-or-rebase history-shape warnings.

## Safety Considerations

Validation should avoid requiring a specific command sequence and should use only final content and observable commit graph shape.

## Test Strategy

Add tests for preferred integration pass, alternate integration warning, missing content failure, and unresolved conflict failure.

## Risks

A single exercise may be less clear than two separate exercises if it tries to teach both workflows at once.

## Deferred Questions

- Should this be one exercise or two?
- Should the preferred workflow be merge-first or rebase-first?
- Should alternate valid history shape warn or fail?
