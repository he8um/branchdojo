# bisect-basic

## Status

Deferred to v0.5 or later. Not implemented in v0.4.

## Difficulty

Advanced.

## Estimated Time

20-30 min.

## Purpose

Practice finding the commit that introduced a regression using deterministic local repository state.

## Starting State

A linear history contains one commit that changes a simple file from a passing state to a failing state.

## User Task

Identify the bad commit and record the answer in the expected final state.

## Expected Final State

The repository is back on the expected branch, the working tree is clean, and the expected answer identifies the seeded bad commit.

## Required Validation Checks

- Metadata exists and is valid.
- Git repository exists.
- Current branch is the expected branch.
- Working tree is clean.
- No bisect, merge, rebase, cherry-pick, or revert operation is active.
- Recorded bad commit matches the expected commit.
- Expected files remain present.

## Warning Conditions

- The bad commit is correct but the repository is left on a detached HEAD.
- Extra diagnostic files exist but do not affect the required answer.

## Progress-Aware Hint Ideas

- If the repository is in bisect state, suggest continuing or resetting bisect before checking.
- If HEAD is detached, suggest returning to the expected branch after recording the answer.
- If the answer file is missing, suggest recording the suspected bad commit.

## Report Considerations

Reports should clearly separate the bad-commit answer check from cleanup checks such as branch and working-tree state.

## Safety Considerations

The exercise should avoid requiring `git bisect run` or shell scripts. The regression signal should be represented by files in the repository.

## Test Strategy

Add setup tests, wrong-answer failure tests, correct-answer pass tests, and cleanup warning tests if detached HEAD can still preserve the answer.

## Risks

The exercise can become too procedural if validation depends on how the user found the bad commit.

## Deferred Questions

- Should the answer be recorded by commit hash, subject, or both?
- Should a detached HEAD final state fail or warn?
- Should the exercise include instructions for manual bisect only?
