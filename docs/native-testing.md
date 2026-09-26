# Native testing in mncs-tui

Two layers, matching the framework's role as a projection surface.

## Layer 1 — experiment corpora (values and regression fixtures)

`.github/workflows/corpus/*.json` pin exact behaviors through `mncs
experiment run`: the 21-case `focused.json` regression net over
`examples.focused_tests`, the 7-case `family-status.json` projection
corpus over `examples.family_status`, and the bounded
`backend-repro.json` budget-exhaustion fixture. Corpora are the only
layer that pins multi-backend execution evidence.

## Layer 2 — native `mncs-test` suites (laws and projection contracts)

`tests/native/*.mncs` (profile 0.18 `test` declarations over the 0.8
framework) pin LAWS:

- `framework_properties` (8 tests): the three focused aggregators as
  native regression tests; space conservation over a 24-case generative
  sweep (`replay` with fixed seeds); clipping containment with exact
  clipped extents; focus traversal totality (`init` lands on widget 2,
  `next` on 3); list-selection clamping at both ends; empty-damage
  identity for identical frames.
- `status_projection` (9 tests): verdict code round-trips including the
  UNKNOWN default; UNKNOWN never equal to FAIL; empty-snapshot zero
  counts; mixed-snapshot partition; injective P/F/? glyphs;
  determinism; selection navigation clamps (down/up/empty).

Run them (four library roots — the repo root carries `examples.*` and
`tests.*` namespaces):

```bash
export MNCS_LIBRARY_PATH="<mncs-test>/native:$PWD/src:$PWD:<mncs-language>/library"
<mncs> test tests/native/framework_properties.mncs --format text
<mncs> test tests/native/status_projection.mncs --format text
```

`--format json` emits the result contract (per-test identity
`mncs:0.2:test-case:<module>::<name>::<hash>`, expected/actual,
assertion code, backend, artifact identity). Failing results import
into Debug via `mncs-debug import-test <result.json>` (proven with a
mutated projection expectation: witness with 7 artifacts).

## Filename rule (learned 2026-09-26)

Module segments must match file names: `examples/focused-tests.mncs`
(renamed to `focused_tests.mncs` this campaign) was directly runnable
but unimportable (`MNE173`), because the resolver maps
`examples.focused_tests` to `examples/focused_tests.mncs`. Keep file
names and module segments identical (underscores, never hyphens).

## Tooling note: Doctor's profile registry lags 0.18

`mncs-doctor doctor` reports `[DOC102|error] unknown profile 0.18` on
`tests/native/*.mncs`. The headers are correct (toolchain runs them;
`mncs-test`'s self-suite declares 0.18). Do not lower the headers:
older profiles cannot declare `test`s.

## Adding coverage

- New framework law: add a `test` to the matching native suite with the
  next free 80xx/81xx assertion code; extend the focused example only
  if a pinned value is also needed.
- New projection: extend `examples/family_status.mncs` (codes in,
  verdicts/counts/glyphs/selection out — never new authority) plus
  corpus cases plus native projection tests.
- Snapshot changes stay in `scripts/snapshot_family.py`, never in MNCS.
