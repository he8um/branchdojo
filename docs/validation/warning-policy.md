# Warning Policy

Warnings exist for valid but non-ideal solutions.

## When to Use Warning

Use `WARNING` when:

- All required final-state checks pass.
- The solution may not match the intended workflow.
- The repository state is acceptable.
- The user should learn a better workflow, but should not be failed.

## When Not to Use Warning

Do not use warnings when:

- Required content is missing.
- Working tree is dirty.
- Conflict markers remain.
- Required branches are missing.
- Metadata is missing.
- Git state is corrupted.

Those are failures.

## Examples

### conflict-basic

Final file content is correct, working tree is clean, but no merge commit is detected.

Result: `WARNING`.

### revert-mistake

Safe config is restored and the bad commit remains, but the fix is not a conventional revert commit.

Result: `WARNING`.

### wrong-branch-commit

Main and feature branch contents are correct, but history shape is unusual.

Result: `WARNING`.

## Warning Output

Warnings should include a learning-oriented next step:

```text
Your solution is acceptable. For the intended workflow, try solving this again using git revert.
```
