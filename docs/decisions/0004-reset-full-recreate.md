# ADR 0004: Reset Uses Full Recreate

## Status

Accepted for v0.1.

## Context

Reversing arbitrary Git states is complex and error-prone.

## Decision

Reset fully recreates the exercise after validating the workspace guard.

## Consequences

Reset behavior is deterministic and easier to test. It must be heavily guarded to avoid data loss.
