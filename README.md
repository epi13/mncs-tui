# mncs-tui

Machine-native terminal UI framework for MNCS, built around first-class geometry, constraint-based layout, structured rendering, and efficient terminal interaction.

> **Status: experimental semantic vertical slice.** `mncs-tui` has validated MNCS source for typed geometry, bounded constraint solving, structured cells/frames, damage projection, generic events, composable widgets, focus, hit-testing, and terminal command planning. The integrated demo `examples/full-demo.mncs` exercises those relationships, but this repository does not yet provide host terminal I/O or claim a production-ready runtime. APIs remain experimental and not yet stable.

## Why this project exists

Terminal UIs are a useful stress test for a machine-native language because they combine continuously changing state with strict spatial relationships. A conventional implementation often reduces a view to strings too early and then reconstructs widths, offsets, clipping, focus, and damage by hand.

`mncs-tui` keeps the semantic structure visible for as long as possible:

```text
application state
      ↓
widget tree + event state
      ↓
constraint graph and resolved rectangles
      ↓
structured cell/frame model
      ↓
diff against the previous frame
      ↓
terminal projection
```

The important fact is the resolved geometry. A widget has a rectangle, a parent, siblings, constraints, focus state, and rendering intent before it becomes terminal text. Resizing therefore re-solves a spatial model instead of making every application recalculate character offsets.

## Design commitments

- **Geometry is first-class.** Points, sizes, rectangles, anchors, clipping regions, and hit regions remain typed data.
- **Layout is constraint-based.** Fixed, intrinsic, minimum/maximum, proportional, and grow/shrink behavior are expressed as constraints and solved centrally.
- **Rendering is structured and diff-based.** Widgets produce semantic cell/frame data; the terminal adapter computes a bounded damage set against the previous frame.
- **Input is an event stream.** Key, mouse, paste, resize, focus, and terminal lifecycle events are typed observations with explicit normalization boundaries.
- **Widgets are composable.** Containers, text, lists, tables, panes, overlays, forms, and status regions share lifecycle and geometry contracts.
- **Uncertainty stays visible.** Unsupported terminal features, ambiguous input, layout overflow, and incomplete upstream language support are reported rather than silently approximated.
- **The language remains the authority.** Missing spatial or event semantics should feed back into `mncs-language`; editor/agent affordances should feed back into `mncs-language-service`.

## Repository layout

```text
src/
  geometry.mncs     hit-regions, viewports, anchors, and clipping over mncs.core.geometry.v1
  layout.mncs       bounded constraint solver (fixed/intrinsic/grow, min/max, remainder)
  render.mncs       cells, frames, branchless damage via vec/mask/select
  events.mncs       typed Key/Mouse/Resize/Paste/Focus/Quit with Unknown preservation
  widgets.mncs      container/text/list/table/status with intrinsic sizing and focus
  focus.mncs        focus identity, traversal, hit-testing, and event dispatch
  terminal.mncs     lifecycle, alt-screen, cursor, style, and damage→commands
  charts.mncs       bounded sparkline levels and glyph semantics
examples/
  first-layout.mncs small rectangle fixture (bootstrap)
  full-demo.mncs    integrated demo exercising layout→render→diff→focus→terminal
  focused-tests.mncs bounded semantic regressions for layout, render, tree, and events
  backend-repro.mncs minimized research-backend regression fixture
docs/
  architecture.md   semantic pipeline and ownership boundaries
  roadmap.md        staged implementation plan and explicit non-goals
  contributing.md   development workflow and upstream feedback rules
```

`src/` modules are now substantial MNCS implementations (Profile 0.8, bounded data, `select`/`vec`/`mask`). The upstream `mncs.core.geometry.v1` (in `mncs-language/library/core/geometry.mncs`) is the canonical primitive store; `mncs.core.partition.v1` owns weighted integer allocation; and `mncs.std.ansi.v1` owns ANSI/VT sequence meaning. `mncs_tui.geometry`, `mncs_tui.layout`, and `mncs_tui.events` adapt those authorities with TUI-specific contracts rather than duplicating them.

The `host/` workspace member is the reusable Unix terminal realization. It owns raw mode,
alternate-screen lifecycle, input decoding, resize observation, and diffed writes for structured
frames; applications still own their semantic state and use the MNCS framework vocabulary for
layout, widgets, focus, and bounded charts.

## The target model

An application should be able to describe a view in terms close to this:

```text
window
 ├─ header   height: intrinsic
 ├─ body     height: grow
 │   ├─ nav  width: 24
 │   └─ main width: grow
 └─ status   height: 1
```

The framework should retain the relationships that this description implies:

- `body` owns the remaining vertical space;
- `nav` is left of `main` and has a minimum width;
- `status` occupies one row at the bottom;
- every child rectangle is clipped by its parent;
- a resize invalidates layout and only the affected frame regions;
- input dispatch follows hit-testing and focus relationships, not raw coordinates in application code.

## Language and service feedback loop

`mncs-tui` deliberately has two upstream feedback channels.

### `mncs-language`

Use this repository to identify language-level pressure such as:

- recursive or aggregate geometry values and invariants;
- bounded constraint solving and overflow behavior;
- identity-preserving tree transformations;
- explicit terminal/resource effects;
- event-state transitions and cleanup guarantees;
- diff correctness and backend realization contracts.

When the framework needs a semantic capability, propose it in `mncs-language` and consume the authoritative API here. Do not create a second geometry, effect, or verification ontology in this repository.

### `mncs-language-service`

Use the service repository for resident analysis and interaction needs such as:

- `.mncs` syntax and semantic diagnostics;
- navigation across widget/layout declarations;
- hover and context packets for geometry relationships;
- candidate analysis for alternative layout or rendering designs;
- semantic tokens and editor support for the TUI vocabulary.

The service may index and explain the framework, but it must not become the semantic authority for layout or terminal behavior.

## Development

The language is evolving, so begin with source review and fixture validation:

```bash
git clone https://github.com/epi13/mncs-tui.git
cd mncs-tui
```

For local source experiments, use a sibling checkout of [`mncs-language`](https://github.com/epi13/mncs-language), build its CLI, and set `MNCS_LIBRARY_PATH` to the language `library` followed by this repository's `src` directory:

```bash
cargo build --release -p mncs-cli --manifest-path ../mncs-language/Cargo.toml
MNCS_LIBRARY_PATH="$PWD/../mncs-language/library:$PWD/src" \
  ../mncs-language/target/release/mncs validate examples/full-demo.mncs
```

The focused source fixtures and the CI workflow use one source path per validation invocation. The research-bytecode backend is currently bounded by an explicit minimized regression fixture; the semantic body request remains the executable evidence for the layout result.

Before proposing a change, read [the architecture](docs/architecture.md) and [contributing guide](docs/contributing.md). Every new capability should state whether it is an application concern, a framework concern, a language concern, or a service concern.

## Non-goals for the bootstrap

This first scaffold does not promise:

- a stable public API or compatibility policy;
- a complete terminal emulator or escape-sequence implementation;
- a production-grade constraint solver;
- unrestricted concurrency or unbounded event processing;
- automatic terminal capability detection without explicit evidence;
- visual output being equivalent to semantic correctness;
- replacing Bubble Tea, Bubblewrap, or other mature terminal UI systems today.

The goal is a coherent proving ground for whether MNCS can make spatial relationships, rendering deltas, event effects, and verification obligations easier to express, inspect, and evolve.
