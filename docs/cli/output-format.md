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
Score: 8/8

Checks:
✅ Metadata exists
✅ Current branch is main
✅ Working tree is clean
✅ No merge state is active
✅ app.txt exists
✅ Conflict markers removed
✅ Expected headline exists
✅ Expected CTA exists
```

## Warning Example

```text
BranchDojo Result

Exercise: conflict-basic
Status: WARNING
Score: 8/8

Checks:
✅ Metadata exists
✅ Current branch is main
✅ Working tree is clean
✅ Required content exists
⚠️ Final state is valid, but no merge commit was detected

Next:
Your solution is acceptable. For the intended workflow, try solving it again with a merge commit.
```

## Failed Example

```text
BranchDojo Result

Exercise: conflict-basic
Status: FAILED
Score: 5/8

Checks:
✅ Metadata exists
✅ Current branch is main
❌ Working tree is clean
❌ Conflict markers removed
✅ app.txt exists

Next:
Run git status, finish the merge resolution, remove conflict markers from app.txt, then run branchdojo check --path . again.
```
