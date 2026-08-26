# mncs-tui

Machine-native terminal UI framework for MNCS, built around first-class geometry, constraint-based layout, structured rendering, and efficient terminal interaction.

> **Status: experimental bootstrap.** `mncs-tui` is a design and language-pressure project. The initial source modules are MNCS-family fixtures and are expected to evolve with the language. This repository does not claim a stable widget API, a final terminal backend, or production suitability.

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
  geometry.mncs     points, sizes, rectangles, and spatial identity
  layout.mncs       axes, sizing constraints, and placements
  render.mncs       structured cells, frames, and damage concepts
  events.mncs       terminal input and lifecycle event vocabulary
  widgets.mncs      widget identity, kinds, and composition seams
examples/
  first-layout.mncs small MNCS source fixture that constructs a rectangle
docs/
  architecture.md   semantic pipeline and ownership boundaries
  roadmap.md        staged implementation plan and explicit non-goals
  contributing.md   development workflow and upstream feedback rules
```

The `.mncs` files under `src/` are intentionally small. They establish the vocabulary and pressure points before the language has a finalized package/module system for a reusable UI library.

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

For local source experiments, use a sibling checkout of [`mncs-language`](https://github.com/epi13/mncs-language) and run its normal validation commands against the fixtures. The exact command is intentionally not frozen here while the language CLI and package model are changing.

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
