# Learning Paths

## Status

Documentation-only planning baseline. BranchDojo does not provide CLI learning-path commands yet.

## Beginner Path

Recommended order:

1. `conflict-basic`
2. `revert-mistake`
3. `wrong-branch-commit`

This path practices conflict resolution, safe history repair, and moving work to the intended branch.

## Intermediate Path

Recommended order:

1. `stash-switch`
2. `cherry-pick-basic`
3. `detached-head-recovery`

This path practices preserving local changes, selectively moving commits, and recovering detached HEAD work.

## Advanced Path

Current and planned v0.4 advanced practice:

1. `interactive-rebase-basic` - implemented on main as unreleased v0.4 work.
2. `bisect-basic`
3. `merge-vs-rebase`
4. `tag-release-fix`

The remaining candidates are design candidates. v0.4 should implement only the candidates that remain deterministic, safe, and compatible with final-state validation.

## Non-Goals

- No CLI learning-path support in this planning baseline.
- No classroom dashboard.
- No analytics or telemetry.
- No remote tracking.
- No external exercise packs.
