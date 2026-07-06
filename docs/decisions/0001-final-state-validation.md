# ADR 0001: Final-State Validation

## Status

Accepted for v0.1.

## Context

Exact command tracking is fragile, platform-specific, and inconsistent with the goal of accepting multiple valid Git workflows.

## Decision

BranchDojo validates the final repository state instead of tracking exact command sequences.

## Consequences

Validators focus on branch state, file content, clean working tree, metadata, and history requirements. Non-ideal but valid workflows may return WARNING instead of FAILED.
