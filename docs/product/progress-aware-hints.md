# Progress-Aware Hints

## Status

Design proposal for v0.3. Not implemented.

## Goal

Make hints more useful by reflecting current repository state while keeping BranchDojo deterministic and practice-first.

## Current Behavior

`branchdojo hint --path <path>` prints static exercise hints from the exercise definition.

## Proposed Behavior

Hints should combine:

- Shared state-aware guidance for common blockers.
- Exercise-specific guidance for the intended workflow.

Examples:

- Dirty working tree: suggest committing, stashing, or cleaning pending changes as appropriate for the exercise.
- Wrong branch: mention the expected final branch.
- Conflict markers: mention resolving markers and committing the result.
- Detached HEAD: mention creating or switching to the expected recovery branch.
- Missing expected branch: mention creating or restoring the branch.

## Constraints

- Do not inspect shell history.
- Do not track exact command sequence.
- Do not mutate the repository.
- Do not require network access.
- Keep hints short enough to be read in the terminal.

## Possible Design

Add a shared hint-state analyzer that reads repository facts already used by validators:

- current branch
- working tree status
- active Git operation
- expected branches
- expected files
- conflict marker presence where relevant

Each exercise can then map these facts to a small ordered hint list.

## Safety Impact

Hint generation must remain read-only. Any Git calls must use explicit arguments and the workspace path as `current_dir`.

## Testing

- Unit-test hint-state analysis.
- Integration-test state-aware output for representative exercise states.
- Regression-test static fallback hints for all released exercises.
