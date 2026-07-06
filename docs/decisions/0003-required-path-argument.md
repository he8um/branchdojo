# ADR 0003: Required Path Argument

## Status

Accepted for v0.1.

## Context

Implicit current-directory behavior can lead to accidental modifications in the wrong repository.

## Decision

Commands that operate on workspaces require an explicit --path argument in v0.1.

## Consequences

The CLI is slightly more verbose but safer and clearer for beginners.
