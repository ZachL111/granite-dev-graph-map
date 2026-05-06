# granite-dev-graph-map

`granite-dev-graph-map` keeps a focused Rust implementation around developer tools. The project goal is to build a Rust toolkit that studies graph behavior through transition tables, with invalid-transition tests and no network dependency.

## Reason For The Project

This is intentionally local and self-contained so it can be inspected without credentials, services, or seeded history.

## Granite Dev Graph Map Review Notes

For a quick review, compare `change width` with `change width` before reading the middle cases.

## What It Does

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/granite-dev-graph-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `change width` and `change width`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## How It Is Put Together

The core code exposes a scoring path and the added review layer uses `signal`, `slack`, `drag`, and `confidence`. The domain terms are `change width`, `diagnostic quality`, `review cost`, and `safe rewrite`.

The Rust code keeps the review rule close to the tests.

## Run It

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Check It

The check exercises the source code and the review fixture. `stale` is the high score at 238; `baseline` is the low score at 137.

## Boundaries

No external service is required. A deeper version would add more negative cases and a clearer boundary around invalid input.
