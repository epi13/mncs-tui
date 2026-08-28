# Roadmap

The roadmap is a research sequence. “Scaffolded” means vocabulary and boundaries exist; it does not mean the capability is production-ready.

## Phase 0 — Framework bootstrap

**Status: scaffolded.**

- establish geometry, layout, frame, event, and widget subjects;
- document the string-last pipeline;
- define ownership boundaries with `mncs-language` and `mncs-language-service`;
- keep early MNCS source fixtures small and explicit about experimental status.

## Phase 1 — Geometry and layout corpus

**Status: experimental — exercised via `mncs.core.geometry.v1`, `mncs.core.partition.v1`, and `mncs_tui.layout`.**

- `library/core/geometry.mncs` provides total Point/Size/Rect/Insets with containment, intersection, union, clipping, and `select` branchless helpers (Profile 0.8, validated).
- `src/geometry.mncs` adds hit-regions, viewports, anchors (`mncs.core.geometry.v1` + bounded `[HitRegion; 4]` sequence-typed hit-test/visibility helpers, with four-argument wrappers for existing call sites).
- `mncs.core.partition.v1` owns exact weighted quotient/remainder arithmetic, including overflow-safe large products and deterministic remainder assignment.
- `src/layout.mncs` implements a bounded one-dimensional solver for 4 constraints (fixed/intrinsic/grow, min/max, deterministic remainder) and composes into `DemoLayout` (`header 3/body grow/status 1`, `nav 24/main grow`). Conflicting fixed/grow minima report overflow instead of being silently accepted.

## Phase 2 — Structured frame and diff model

**Status: experimental — exercised via `src/render.mncs`.**

- `Cell`/`Frame` (bounded 4-cell frame) with `frame_get`/`frame_set` via `select`.
- `glyph_mask_changed`/`style_mask_changed` use `vec<i64,4>` + `vec_ne` → `mask<4>` + `mask_any`/`mask_none`.
- `frame_damage` is a bounded `iterate` scan that expands a damage `Rect` via `rect_union` and counts changed cells branchlessly. Full redraw on resize.

## Phase 3 — Event and lifecycle boundary

**Status: experimental — exercised via `mncs.std.ansi.v1` + `src/events.mncs` + `src/focus.mncs` + `src/terminal.mncs`.**

- `KeyEvent`/`MouseEvent`/`Resize`/`Paste`/`Focus`/`Quit`/`Unknown` payload sums (Profile 0.6) with `Unknown` preservation (evidence-honest).
- `mncs.std.ansi.v1` parses bounded ANSI/VT sequences into generic `AnsiEvent` values; `src/events.mncs` maps those values into TUI events and preserves `Unknown`.
- `dispatch_*` respects focus and hit-testing (`HitRegion` from `src/geometry.mncs`) and returns `DispatchResult { consumed, new_focus, damage }`.
- `src/terminal.mncs` is a pure state machine (`terminal_init`/`enter_alt`/`resize`/`cleanup`) with `generation` and `damage_to_commands`.

## Phase 4 — Composable widgets

**Status: experimental — exercised via `src/widgets.mncs`.**

- `Widget { id, kind, parent, rect, focusable, visible }` participates in measurement (`intrinsic_size`), layout (`widget_clipped_rect`), rendering (`widget_frame`/`list_frame`), clipping, focus, and state transitions (`ListState`/`TableState`).
- `WidgetTree` bounded to four widgets; `next_focusable` is a bounded `iterate` with `select`.
- List/table/status/pane/overlay/form are present as kinds with real branches.

## Phase 5 — Terminal realization

**Status: experimental — pure projection adapter in `src/terminal.mncs`.**

- Pure `TerminalState` with `alt`/`cursor_visible`/`initialized`/`generation`.
- `damage_to_commands` maps `Damage`+`Frame` to bounded `TerminalCommand` values, explicitly clearing disappeared cells and forcing a clear-prefix full redraw after resize.
- Cleanup is deterministic (`terminal_cleanup` restores alt and cursor).

The current research-bytecode backend still exhausts a bounded execution budget on the minimized `examples/backend-repro.mncs` import path. CI records that fact as a regression while the semantic body request returns the expected allocation. This is an upstream backend/performance issue, not a reason to weaken the layout assertion.

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
