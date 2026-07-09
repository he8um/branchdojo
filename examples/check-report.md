# Example Output: Markdown Check Report

```markdown
# BranchDojo Check Report

## Summary

- Exercise: `conflict-basic`
- Title: Resolve a basic merge conflict
- Difficulty: beginner
- Category: Merge conflicts
- Estimated time: 10-15 min
- Expected final branch: `main`
- Status: FAILED
- Score: 9 / 11

## Result

BranchDojo validates the final repository state, not the exact command sequence.

## Checks

| Severity | Status | Check | Details |
|---|---|---|---|
| required | passed | Metadata exists |  |
| required | failed | Expected CTA exists |  |

## Next Steps

- Resolve failed required checks.
- Open app.txt, remove conflict markers, preserve both required lines, then run branchdojo check --path . again.
- Run `branchdojo hint --path ./dojo-conflict-basic` for progress-aware guidance.
- Run `branchdojo check --path ./dojo-conflict-basic` again.
```
