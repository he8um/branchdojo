# tag-release-fix

## Status

Design candidate for v0.4. Not implemented.

## Difficulty

Advanced.

## Estimated Time

20-30 min.

## Purpose

Practice repairing a local release state and placing the expected release tag on the corrected commit.

## Starting State

A release branch contains an incorrect release state, and a fix commit or fix branch contains the corrected content.

## User Task

Move the release branch to the corrected content and create the expected release tag locally.

## Expected Final State

The repository is on the release branch, the corrected content exists, the expected release tag points at the corrected commit, and the working tree is clean.

## Required Validation Checks

- Metadata exists and is valid.
- Git repository exists.
- Current branch is the expected release branch.
- Working tree is clean.
- No active Git operation remains.
- Corrected release content exists.
- Incorrect release content is absent.
- Expected tag exists.
- Expected tag points to the corrected commit.

## Warning Conditions

- A lightweight tag is used when an annotated tag is preferred.
- Corrected content exists but release history includes unclear extra repair commits.

## Progress-Aware Hint Ideas

- If the expected tag is missing, suggest inspecting release branch history and creating the tag.
- If the tag points to the wrong commit, suggest checking the corrected commit before moving the tag.
- If the working tree is dirty, suggest committing or discarding local changes before checking.

## Report Considerations

Reports should include tag-target checks and any annotated-tag warning separately.

## Safety Considerations

The exercise must remain local. It must not require remote tags, force-pushing, or publishing a release.

## Test Strategy

Add tests for corrected annotated tag pass, lightweight tag warning, wrong tag target failure, and missing corrected content failure.

## Risks

Tag repair workflows can imply remote operations. The exercise must clearly stay inside a disposable local repository.

## Deferred Questions

- Should annotated tags be required or only preferred?
- Should the exercise include a preexisting incorrect tag?
- Should moving an existing tag be allowed in the intended solution?
