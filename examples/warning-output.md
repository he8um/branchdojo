# Example Output: Warning

```text
BranchDojo Result

Exercise: revert-mistake
Status: WARNING
Score: 10/10

Checks:
✅ Metadata exists
✅ Git directory exists
✅ Current branch is main
✅ Working tree is clean
✅ No merge/rebase/cherry-pick/revert state is active
✅ config.txt exists
✅ Unsafe config is absent
✅ Safe config exists
✅ Bad commit still exists
✅ Fix commit exists after bad commit
⚠️ Revert-style workflow check

Next:
Restore config.txt to the safe value while keeping the bad commit in history, then run branchdojo check --path . again.
If the result is a warning, try solving this again using git revert.
```
