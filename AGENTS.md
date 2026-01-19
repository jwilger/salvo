# Working Agreement for Salvo

This document defines the binding process, architectural style, and development discipline for this codebase. It is not aspirational. These rules are expected to be followed unless explicitly revised.

## Architectural Decision Records

- Any **major architectural decision** must be captured as an ADR under `docs/adrs/`
- After an ADR is accepted, `docs/ARCHITECTURE.md` must be updated so it reflects the **current, holistic architectural understanding**
- `docs/ARCHITECTURE.md` is a living document; it represents the present state, not historical intent

## Architectural Style

- Overall style: **Functional Core / Imperative Shell**
- The functional core:
  - Contains all business rules and game logic
  - Is deterministic and referentially transparent
  - Has no direct side effects
- The imperative shell:
  - Handles all I/O, persistence, randomness, and user interaction
  - Invokes the core via explicit inputs and receives explicit outputs

## Effects and Side Effects

- All side effects are pushed to the imperative shell
- Interaction between shell and core uses an **Effect pattern**
- Effects are explicit in types and data flow
- No hidden side effects or implicit global state

## Control Flow and Error Handling

- Use **Railway‑Oriented Programming** for control flow and error handling (per Scott Wlaschin)
- Errors are values
- Happy paths and error paths are explicit and visible in types

## Domain Modeling

- Use **semantic domain types** throughout the functional core
  - Avoid primitive obsession
  - Structural types may be used only as internal building blocks of domain types
- Invalid states must be unrepresentable inside the core
- Apply **parse‑don’t‑validate** at system boundaries

## Testing Philosophy

- Development follows **strict Test‑Driven Development (Kent Beck)**
- Tests focus on **observable behavior**, not implementation details
- Compiler errors count as test failures

## TDD Cycle Rules

The TDD cycle is enforced step‑by‑step. Do not batch steps.

### Red Phase

- Write **exactly one failing test**
- Run the test and confirm it fails in the expected way
- Compiler errors are valid red failures
- Write the code you *wish you had*
- Missing types and APIs are acceptable
- Stop after writing the test and wait for review before proceeding

### Green Phase

- Fix **only the current failing error**
- Do not address errors that have not yet been observed
- Write the **minimal implementation** needed to pass the test as written
- Do not write code unrelated to making the test pass
- Ensure the implementation moves toward the agreed architecture
- Stop after the implementation and wait for review

### Refactor Phase

- Refactoring is an explicit phase
- Definition of refactoring:
  - **All previously passing tests continue to pass**
  - The **intent of all tests remains unchanged**
- The assistant’s role:
  - Suggest possible refactorings
  - Explain the motivation and expected benefit
- The human’s role:
  - Decide which refactorings, if any, should be pursued
- No refactoring is performed without explicit approval

## Scope of Enforcement

- These rules apply to **all code** in the repository:
  - Functional core
  - Imperative shell
  - Persistence
  - CLI/TUI
  - Tests

## Version Control Discipline

- Use **Conventional Commits** format for all commits
- Commit messages must explain **why** the change was made, not just what or how
- Create a commit whenever:
  - All tests are passing at the end of a Green phase
  - Before entering a Refactor phase
  - After each refactoring step, as long as all tests continue to pass

## Operating Principle

- Correctness, clarity, and architectural integrity outweigh convenience
- Speed is achieved through disciplined iteration, not shortcuts
- When in doubt, stop and ask before proceeding
