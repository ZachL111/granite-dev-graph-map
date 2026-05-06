# granite-dev-graph-map

`granite-dev-graph-map` packages a practical developer tools exercise in Rust. The emphasis is on deterministic behavior, a small public API, and examples that explain the tradeoffs.

## How I Read Granite Dev Graph Map

The useful thing to inspect here is how the same score rule is represented in code, metadata, and examples. If those three pieces disagree, the audit script should make the drift visible.

## Problem Shape

I use this kind of project to make a rule visible before adding more machinery around it. The important part here is not the size of the codebase. It is that the input signals, scoring rule, fixture data, and expected output can all be checked in one sitting.

## Repository Map

- `src`: primary implementation
- `tests`: verification harness
- `fixtures`: compact golden scenarios
- `examples`: expanded scenario set
- `metadata`: project constants and verification metadata
- `docs`: operations and extension notes
- `scripts`: local verification and audit commands
- `Cargo.toml`: Rust package metadata

## Main Behaviors

- Includes extended examples for safe defaults, including `surge` and `degraded`.
- Documents repeatable output tradeoffs in `docs/operations.md`.
- Runs locally with a single verification command and no external credentials.
- Stores project constants and verification metadata in `metadata/project.json`.
- Adds a repository audit script that checks structure before running the language verifier.

## Internal Model

The interesting part is the boundary between accepted and reviewed scenarios. Extended examples sit near that boundary so future edits can show whether the model became more permissive or more cautious. The Rust code keeps ownership and data movement plain, which makes the tests useful for checking both behavior and API shape.

## Run It Locally

Clone the repository, enter the directory, and run the verifier. No database server, cloud account, or token is required.

## Scenario Walkthrough

The extended cases are not random smoke tests. `degraded` keeps pressure on the review path, while `surge` shows the model when capacity and weight are strong enough to clear the threshold.

## How To Run It

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

This runs the language-level build or test path against the compact fixture set.

## Validation

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/audit.ps1
```

The audit command checks repository structure and README constraints before it delegates to the verifier.

## Follow-Up Work

- Add a loader for `examples/extended_cases.csv` and promote selected cases into the language test suite.
- Add a short report command that prints the score breakdown for a single scenario.
- Add malformed input fixtures so the failure path is as visible as the happy path.
- Add one more developer tools fixture that focuses on a malformed or borderline input.

## Known Edges

The repository favors determinism over breadth. It does not pull live data, keep secrets, or depend on network access for verification.
