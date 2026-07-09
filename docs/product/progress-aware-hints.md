# Progress-Aware Hints

## Status

Implemented for v0.3 development.

## Goal

Make hints more useful by reflecting current repository state while keeping BranchDojo deterministic and practice-first.

## Behavior

`branchdojo hint --path <path>` combines:

- Shared state-aware guidance for common blockers.
- Exercise-specific guidance for the intended workflow.
- Static exercise hints as a general fallback.

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

The shared hint-state analyzer reads repository facts already used by validators:

- current branch
- working tree status
- active Git operation
- expected branches
- expected files
- conflict marker presence where relevant

Each exercise maps these facts to a small ordered hint list. Hints are intentionally lightweight and do not duplicate every validator rule.

## Safety Impact

Hint generation must remain read-only. Any Git calls must use explicit arguments and the workspace path as `current_dir`.

Hints require a valid BranchDojo workspace. Missing or invalid `.branchdojo.json` is refused before any hint analysis.

## Testing

- Unit-test catalog drift between exercise definitions and metadata.
- Integration-test state-aware output for representative exercise states.
- Regression-test general fallback hints for all released exercises.
