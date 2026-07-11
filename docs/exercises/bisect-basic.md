# bisect-basic

## Status

Release-candidate ready for v0.5.

## Difficulty

Advanced.

## Category

Debugging.

## Estimated Time

20-30 min.

## Purpose

Practice identifying the commit that introduced a regression using Git history investigation.

## Starting State

The repository starts on `main` with a clean working tree and a short linear history:

1. `Initialize sample app`
2. `Add stable login flow`
3. `Add checkout calculation`
4. `Introduce discount regression`
5. `Update copy text`
6. `Refactor display labels`

The regression is represented by local files. For example, `check.txt` or `app.txt` changes from `discount_total=correct` to `discount_total=incorrect`.

## User Task

Identify the regression-introducing commit and record the answer in `diagnosis.md`.

## Expected Final State

The repository is on `main`, the working tree is clean, no active bisect or other Git operation remains, and `diagnosis.md` identifies the seeded culprit commit.

The regression does not need to be fixed for this exercise.

## Required Validation Checks

- Metadata exists and is valid.
- Git repository exists.
- Current branch is the expected branch.
- Working tree is clean.
- No bisect, merge, rebase, cherry-pick, or revert operation is active.
- `app.txt` contains `discount_total=incorrect`.
- `check.txt` contains `expected_discount_total=correct`.
- `diagnosis.md` exists.
- Recorded culprit matches `Introduce discount regression` by subject, stable regression marker, or resolvable commit hash.
- The culprit commit is reachable from `main`.
- Expected files remain present.

## Warning Conditions

- The answer is correct but uses a valid hash without the commit subject.
- The answer is correct but lacks supporting detail.
- A diagnostic branch exists even though the final branch is correctly `main`.

## Failure Conditions

- Diagnosis artifact is missing.
- Diagnosis names the wrong commit.
- Working tree is dirty.
- Repository is on the wrong branch if final design requires `main`.
- A bisect or other Git operation remains active.
- Expected files, fixture markers, or metadata are missing.

## Progress-Aware Hints

- If the repository is in bisect state, suggest continuing or resetting bisect before checking.
- If HEAD is detached, suggest returning to the expected branch after recording the answer.
- If the answer file is missing, suggest recording the suspected bad commit.
- If the recorded answer is wrong, suggest comparing the good and bad file markers across history.

## Report Considerations

Reports should clearly separate diagnosis checks from cleanup checks such as branch, working-tree, and active-operation state.

## Safety Considerations

The exercise should avoid requiring `git bisect run` or shell scripts. The regression signal should be represented by files in the repository, with no network access or external dependencies.

## Test Coverage

Covered by tests for deterministic setup, missing diagnosis, wrong diagnosis, correct diagnosis, dirty working tree, wrong branch policy, active bisect state, JSON output, Markdown reports, hints, reset, and no regression for the existing released exercises.

## Risks

The exercise can become too procedural if validation depends on how the user found the bad commit.

Hash-only answers can be brittle if fixture history changes during development.

## Deferred Questions

- Should a fix-after-bisect workflow become a separate future exercise?
