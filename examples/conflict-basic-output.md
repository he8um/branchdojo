# Example Output: conflict-basic Failed

```text
BranchDojo Result

Exercise: conflict-basic
Status: FAILED
Score: 9/11

Checks:
✅ Metadata exists
✅ Git directory exists
✅ Current branch is main
✅ Working tree is clean
✅ No merge/rebase/cherry-pick/revert state is active
✅ app.txt exists
✅ Conflict markers removed
✅ Expected headline exists
❌ Expected CTA exists
✅ Branch `feature/landing-copy` exists
❌ History includes feature work
✅ Merge commit check

Next:
Open app.txt, remove conflict markers, preserve both required lines, then run branchdojo check --path . again.
If the result is a warning, try solving it again with a merge commit.
```
