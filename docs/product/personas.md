# Personas

## Persona 1: Junior Developer

### Git knowledge

- Knows `git add`, `git commit`, `git push`, `git pull`.
- May know branches conceptually.
- Often depends on IDE Git UI.
- Gets nervous when Git reports conflicts or detached states.

### Pain points

- Fear of losing work.
- Does not know how to inspect repository state.
- Does not know which commands are safe.
- Struggles to understand error messages.

### What BranchDojo must provide

- Safe disposable environment.
- Clear exercise goals.
- Practical hints.
- Validation output with next steps.

### UX implications

- Avoid vague errors.
- Always include what happened and what to try next.
- Prefer concrete file/branch names.

## Persona 2: Vibe Coder Developer

### Git knowledge

- Can build with tools and prompts.
- May generate code quickly but lacks low-level Git workflow confidence.
- Often commits large changes or works in a single branch.

### Pain points

- Git is treated as a deployment/push tool, not a workflow tool.
- Conflicts feel like blockers.
- History cleanup feels risky.

### What BranchDojo must provide

- Short realistic exercises.
- Repetition.
- No setup friction.
- Results that explain what final state is missing.

### UX implications

- Keep commands simple.
- Generated exercise README must explain context.
- Hints should not require Git expertise.

## Persona 3: Git-Basic Developer Without Workflow Practice

### Git knowledge

- Knows branches, commits, and merges in theory.
- Has not practiced recovery patterns enough.
- Wants to become safer in team workflows.

### Pain points

- Knows multiple possible commands but not which state is correct.
- Uses risky commands like reset without confidence.
- Cannot evaluate whether the repository is clean and safe.

### What BranchDojo must provide

- Realistic branch states.
- Final-state validation.
- Warnings for non-ideal but valid solutions.

## Persona 4: Bootcamp Instructor or Team Onboarding Owner

### Needs

- Repeatable exercises.
- Easy local setup.
- JSON output for future automation.
- Clear exercise definitions.

### What BranchDojo must provide

- Deterministic setup.
- Exercise catalog.
- Consistent validation output.
- Stable command-line interface.
