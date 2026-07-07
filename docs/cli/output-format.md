# Output Format

BranchDojo output must be clear, actionable, and stable enough for users to understand without reading source code.

## Human Check Output

Format:

```text
BranchDojo Result

Exercise: <exercise-id>
Status: <PASSED|WARNING|FAILED>
Score: <passed>/<total>

Checks:
<icon> <label>
...

Next:
<actionable next step>
```

## Icons

Use simple icons in human output only:

- `✅` for passed checks.
- `⚠️` for warning checks.
- `❌` for failed checks.

JSON output must not contain icons.

## Passed Example

```text
BranchDojo Result

Exercise: conflict-basic
Status: PASSED
Score: 11/11

Checks:
✅ Metadata exists
✅ Git directory exists
✅ Current branch is main
✅ Working tree is clean
✅ No merge/rebase/cherry-pick/revert state is active
✅ app.txt exists
✅ Conflict markers removed
✅ Expected headline exists
✅ Expected CTA exists
✅ Branch `feature/landing-copy` exists
✅ History includes feature work
✅ Merge commit check
```

## Warning Example

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

## Failed Example

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
```
