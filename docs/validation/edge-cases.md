# Edge Cases

## Missing `.branchdojo.json`

Result: error, not validation failure.

Reason: BranchDojo cannot know which exercise to validate safely.

## Invalid `.branchdojo.json`

Result: error.

Reason: Metadata guard is compromised.

## Missing `.git`

Result: failed check or error depending on command.

For `check`, return failed validation if metadata exists but Git repository is missing.

## User Deletes Expected File

Result: `FAILED`.

## User Is on Wrong Branch

Result: `FAILED` if current branch is required for the exercise.

## Working Tree Is Dirty

Result: `FAILED`.

## Conflict Markers Remain

Result: `FAILED`.

## User Rewrites History

Depends on exercise:

- If required historical commit is missing: `FAILED`.
- If final state is correct and history requirement is not required: possible `WARNING`.

## User Renames Branch

If required branch is missing: `FAILED`.

## User Solves with Different Valid Workflow

If final state satisfies required checks: `PASSED` or `WARNING`.

## Line Ending Changes

Validators should normalize line endings. Do not fail solely because of CRLF vs LF.

## User Modifies README.branchdojo.md

Should not affect validation unless exercise files require it. The README is instructional, not part of solution.

## User Runs Check Outside Workspace

Return `BD004`.

## User Runs Reset on Real Repo

Return `BD008` or unsafe workspace error.
