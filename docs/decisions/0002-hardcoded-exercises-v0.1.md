# ADR 0002: Hardcoded Exercises in v0.1

## Status

Accepted for v0.1.

## Context

A config-driven exercise engine would add parsing, schema evolution, and security concerns before the MVP proves the core model.

## Decision

Exercises are implemented as hardcoded Rust modules in v0.1.

## Consequences

The MVP remains deterministic and easier to test. Custom exercise definitions are deferred to v0.4.
