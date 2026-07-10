# merge-vs-rebase

## Status

Implemented in v0.4.

## Difficulty

Advanced.

## Estimated Time

20-30 min.

## Purpose

Practice integrating divergent branches while understanding when history shape matters.

## Starting State

`main` and `feature/pricing-copy` have both moved forward from a shared base:

- `main` contains updated checkout trust copy.
- `feature/pricing-copy` contains two pricing page commits.

## User Task

Bring `feature/pricing-copy` up to date with `main`, integrate the pricing work into `main`, preserve both the checkout update and pricing copy, and leave the repository clean on `main`.

## Expected Final State

The repository is on `main`, the working tree is clean, `main` contains the final pricing and checkout copy, and `feature/pricing-copy` still contains the pricing work. A clean linear integration is preferred.

## Required Validation Checks

- Metadata exists and is valid.
- Git repository exists.
- Current branch is `main`.
- Working tree is clean.
- No active Git operation remains.
- `main` and `feature/pricing-copy` exist.
- `main:pricing.txt` contains the pricing title, headline, FAQ title, and FAQ copy.
- `main:checkout.txt` contains the checkout trust copy.
- `feature/pricing-copy:pricing.txt` contains the pricing headline and FAQ copy.

## Warning Conditions

- Final content is correct but a merge commit was used instead of the preferred linear integration.
- Final content is correct but `feature/pricing-copy` was not updated with the mainline checkout work.
- Final content is correct but the seeded feature commit subjects are not reachable from `main`.
- Final content is correct but the expected history order is not ideal.

## Progress-Aware Hint Ideas

- If the user is still on `feature/pricing-copy`, suggest integrating it into `main`.
- If the source branch is missing, mention restoring `feature/pricing-copy`.
- If `main` is missing pricing or checkout content, mention the missing side of the integration.
- If a merge commit exists with valid final content, mention the preferred linear integration.

## Report Considerations

Reports should show content checks separately from merge-or-rebase history-shape warnings.

## Safety Considerations

Validation should avoid requiring a specific command sequence and should use only final content and observable commit graph shape.

## Test Strategy

Integration tests cover setup, starting failure and JSON shape, linear-history pass, merge-commit warning, wrong branch, dirty working tree, missing pricing content, missing checkout content, missing feature branch, hints, reports, and reset.

## Risks

History-shape validation must stay tolerant enough to accept equivalent final repository states while still warning for non-preferred integration shape.
