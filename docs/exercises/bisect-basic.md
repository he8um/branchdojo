# bisect-basic

## Status

Design candidate for v0.5. Not implemented.

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

Identify the regression-introducing commit and record the answer in an artifact such as `diagnosis.md` or `culprit.txt`.

## Expected Final State

The repository is on the expected final branch, likely `main`, the working tree is clean, no active bisect or other Git operation remains, and the diagnosis artifact identifies the seeded culprit commit.

## Required Validation Checks

- Metadata exists and is valid.
- Git repository exists.
- Current branch is the expected branch.
- Working tree is clean.
- No bisect, merge, rebase, cherry-pick, or revert operation is active.
- Diagnosis artifact exists.
- Recorded culprit matches the expected regression-introducing commit by stable evidence.
- Expected files remain present.

## Warning Conditions

- The bad commit is correct but the repository is left on a detached HEAD.
- Extra diagnostic files exist but do not affect the required answer.
- The answer is correct but uses less-preferred evidence, if final design allows multiple answer forms.

## Failure Conditions

- Diagnosis artifact is missing.
- Diagnosis names the wrong commit.
- Working tree is dirty.
- Repository is on the wrong branch if final design requires `main`.
- A bisect or other Git operation remains active.
- Expected files or metadata are missing.

## Progress-Aware Hint Ideas

- If the repository is in bisect state, suggest continuing or resetting bisect before checking.
- If HEAD is detached, suggest returning to the expected branch after recording the answer.
- If the answer file is missing, suggest recording the suspected bad commit.
- If the recorded answer is wrong, suggest comparing the good and bad file markers across history.

## Report Considerations

Reports should clearly separate diagnosis checks from cleanup checks such as branch, working-tree, and active-operation state.

## Safety Considerations

The exercise should avoid requiring `git bisect run` or shell scripts. The regression signal should be represented by files in the repository, with no network access or external dependencies.

## Test Strategy

Add tests for deterministic setup, missing diagnosis, wrong diagnosis, correct diagnosis, dirty working tree, wrong branch policy, active bisect state, JSON output, Markdown reports, hints, reset, and no regression for all nine released exercises.

## Risks

The exercise can become too procedural if validation depends on how the user found the bad commit.

Hash-only answers can be brittle if fixture history changes during development.

## Deferred Questions

- Should the answer be recorded by commit hash, subject, or both?
- Should a detached HEAD final state fail or warn?
- Should the exercise include instructions for manual bisect only?
- Should v0.5 require only identification, or identification plus a fix?
- Should a fix-after-bisect workflow become a separate future exercise?
