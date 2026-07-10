# tag-release-fix

## Status

Implemented in v0.4.

## Difficulty

Advanced.

## Estimated Time

20-30 min.

## Purpose

Practice correcting a local release tag that points to the wrong commit after a release-blocking fix.

## Starting State

`main` contains a `v1.0.0` release candidate, an early `v1.0.0` tag, and a later release-blocker fix commit.

## User Task

Move or recreate the local `v1.0.0` tag so it points to the fixed release commit. Prefer an annotated tag.

## Expected Final State

The repository is on `main`, the working tree is clean, `v1.0.0` points at a commit containing `release_blocker=false` and `release_ready=true`, and the tagged commit does not contain `release_blocker=true`.

## Required Validation Checks

- Metadata exists and is valid.
- Git repository exists.
- Current branch is `main`.
- Working tree is clean.
- No active Git operation remains.
- `main` exists.
- `VERSION` contains `1.0.0`.
- Corrected release content exists on `main`.
- Expected tag exists.
- Expected tag points to fixed release content.
- Incorrect release-blocker content is absent from the tagged commit.

## Warning Conditions

- A lightweight tag is used when an annotated tag is preferred.
- The tag points to correct content but tag message is missing or not ideal.
- The tag points to correct content but not directly to the seeded fix commit.

## Progress-Aware Hint Ideas

- If the expected tag is missing, suggest inspecting release branch history and creating the tag.
- If the tag points to the wrong commit, suggest checking the corrected commit before moving the tag.
- If the working tree is dirty, suggest committing or discarding local changes before checking.
- If the tag is lightweight but correct, mention annotated tags are preferred for releases.

## Report Considerations

Reports should include tag-target checks and any annotated-tag warning separately.

## Safety Considerations

The exercise must remain local. It must not require remote tags, force-pushing, or publishing a release.

## Test Strategy

Implemented coverage includes setup tests, starting-state failure tests, annotated-tag pass, lightweight-tag warning, missing tag, wrong tag target, wrong branch, dirty working tree, missing fixed content, hints, report output, and reset.

## Risks

Tag repair workflows can imply remote operations. The exercise must clearly stay inside a disposable local repository.

## Deferred Questions

- Should future versions accept a broader range of equivalent release repair histories without warning?
- Should future list output support filtering advanced release recovery exercises?
