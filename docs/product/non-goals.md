# Non-Goals

This document protects BranchDojo from scope creep.

## BranchDojo Is Not a Git GUI

It does not replace tools like GitKraken, Fork, Tower, VS Code Source Control, or GitHub Desktop.

## BranchDojo Is Not a Visualizer

It does not visualize commit graphs in v0.1. Users should use Git commands or external tools if they want graph views.

## BranchDojo Is Not a Guided Trainer

It does not walk the user through every command. It provides exercise context, progress-aware hints, general hints, and final validation.

## BranchDojo Is Not a Cheat Sheet

It should not become a long list of Git commands. Useful commands may appear in generated exercise READMEs, but the product remains practice-first.

## BranchDojo Does Not Validate Exact Commands

It does not inspect shell history, terminal sessions, or exact command sequences.

## BranchDojo Does Not Modify Existing User Repositories

v0.1 only creates disposable generated repositories.

## BranchDojo Does Not Use Network APIs

No GitHub API, no remote operations, no tokens, no secrets.

## BranchDojo Does Not Run Shell Scripts

All Git operations must use explicit command arguments.

## BranchDojo Does Not Provide Custom Exercises in v0.1

Exercises are hardcoded Rust modules in v0.1. Custom definitions are a later roadmap item.

## BranchDojo Does Not Track Command History

Hints, validation, and reports use observable repository state. They do not inspect shell history or exact command sequences.
