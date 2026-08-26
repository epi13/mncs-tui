# Roadmap

The roadmap is a research sequence. “Scaffolded” means vocabulary and boundaries exist; it does not mean the capability is production-ready.

## Phase 0 — Framework bootstrap

**Status: scaffolded.**

- establish geometry, layout, frame, event, and widget subjects;
- document the string-last pipeline;
- define ownership boundaries with `mncs-language` and `mncs-language-service`;
- keep early MNCS source fixtures small and explicit about experimental status.

## Phase 1 — Geometry and layout corpus

**Status: planned.**

- represent rectangles, regions, clipping, and hit testing;
- create a finite corpus of fixed, intrinsic, grow, minimum, maximum, and conflicting layouts;
- record layout results, unresolved obligations, and overflow behavior;
- identify missing language/stdlib support through reproducible fixtures.

## Phase 2 — Structured frame and diff model

**Status: planned.**

- define semantic cells, styles, layers, and frame identities;
- produce deterministic frame snapshots;
- compare frames and report bounded damage regions;
- test wide glyphs, combining marks, clipping, and empty regions as explicit cases.

## Phase 3 — Event and lifecycle boundary

**Status: planned.**

- normalize key, mouse, paste, resize, focus, and lifecycle events;
- make terminal modes and cleanup obligations explicit;
- define bounded update batches and failure behavior;
- connect event transitions to focus, hit testing, invalidation, and rendering.

## Phase 4 — Composable widgets

**Status: planned.**

- implement a small set of container and leaf widget contracts;
- preserve widget identity through layout and frame production;
- add list, table, pane, overlay, form, and status-bar experiments;
- compare framework composition against hand-built terminal implementations.

## Phase 5 — Terminal realization

**Status: planned.**

- implement a narrow terminal adapter;
- encode capability observations and unsupported features explicitly;
- verify cursor/mode restoration and output failure paths;
- measure full redraw versus diff projection under bounded workloads.

## Phase 6 — Language and service integration

**Status: planned.**

- upstream required type, effect, collection, and bounded-computation capabilities to `mncs-language`;
- add language-service fixtures for geometry-aware diagnostics, navigation, and context;
- expose candidate layout analysis without mutating the workspace baseline;
- publish evidence records for language-pressure experiments.

## Explicit non-goals

The project will not freeze a final widget API, claim universal terminal portability, or create a second semantic authority while the language and service are still evolving. Distributed rendering, unrestricted concurrency, and automatic visual correctness claims are also out of scope for the bootstrap.

## Acceptance vocabulary

Use the MNCS-family vocabulary consistently:

- **scaffolded** — structure and design boundary exist;
- **experimental** — a real path exists but semantics or APIs are still moving;
- **exercised** — representative fixtures and checks have run;
- **deferred** — intentionally postponed;
- **blocked/unresolved** — dependent on missing upstream semantics or evidence.
