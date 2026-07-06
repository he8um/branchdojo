# Security Policy

BranchDojo works with local files and Git repositories, so safety is a core requirement.

## Supported Versions

Security fixes will target the latest released version once public releases begin.

## Reporting a Vulnerability

Please report issues that could cause:

- Deletion or modification of user files outside a BranchDojo workspace.
- Reset or overwrite of non-BranchDojo repositories.
- Shell injection or unsafe command execution.
- Modification of global Git configuration.
- Unsafe path traversal.
- Incorrect workspace guard behavior.

## Security Expectations

BranchDojo must:

- Refuse unsafe paths.
- Refuse non-empty target directories for new exercises.
- Refuse reset without a valid `.branchdojo.json` file.
- Avoid shell invocation.
- Invoke Git through `std::process::Command` with explicit args.
- Set only local Git config in generated repositories.

## Non-Goals

BranchDojo is not a sandbox for untrusted code. Users should not run arbitrary scripts from generated repositories.
