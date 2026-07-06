# Final-State Validation

## Decision

BranchDojo validates final repository state, not exact command sequence.

## Reason

Exact command sequence validation is fragile and cross-platform unreliable. Users may solve the same Git problem through multiple valid workflows. BranchDojo should reward a correct safe final state.

## What BranchDojo Checks

- Current branch.
- Working tree cleanliness.
- Active merge/rebase/cherry-pick/revert state.
- File existence.
- File content.
- Branch existence.
- History presence where required.
- Metadata integrity.

## What BranchDojo Does Not Check

- Shell history.
- Exact commands typed.
- Terminal session logs.
- Whether the user followed one exact tutorial path.

## Pass, Warning, Fail

### Passed

All required final-state checks are correct and no warning exists.

### Warning

The final state is acceptable, but the workflow/history shape is not ideal.

### Failed

At least one required final-state check is incorrect.

## Examples

### conflict-basic

If final file content is correct, no conflict markers exist, and the tree is clean, the required checks pass. If no merge commit is detected, BranchDojo may return `WARNING`.

### revert-mistake

If the unsafe config is removed, safe config exists, the bad commit remains in history, and there is a later fix commit, the required checks pass. If the fix does not look like a revert commit, BranchDojo may return `WARNING`.

### wrong-branch-commit

If the accidental content is absent from `main` and present in `feature/profile-page`, the required checks pass. If history shape is unusual, return `WARNING`.
