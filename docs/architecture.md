# Architecture

## Purpose

`mncs-tui` is a machine-native TUI framework experiment. Its central architectural question is whether a terminal UI can preserve semantic state, geometry, constraints, and rendering intent through the whole pipeline instead of collapsing them into strings before the terminal boundary.

The initial architecture is intentionally a set of boundaries and data subjects, not a frozen implementation API.

## Pipeline

```text
┌──────────────────┐
│ application state│
└────────┬─────────┘
         │ update + typed events
         ▼
┌──────────────────┐
│ widget graph      │  identity, parent/child, focus, intent
└────────┬─────────┘
         │ constraints
         ▼
┌──────────────────┐
│ layout resolution │  rectangles, clipping, overflow obligations
└────────┬─────────┘
         │ render intents
         ▼
┌──────────────────┐
│ frame model       │  cells, styles, layers, hit regions
└────────┬─────────┘
         │ previous frame + terminal capabilities
         ▼
┌──────────────────┐
│ diff / projection  │  bounded damage and terminal operations
└────────┬─────────┘
         ▼
      terminal
```

Each boundary should preserve enough identity and provenance to explain what changed. A frame diff is not merely an optimization: it is a structured observation of how state and geometry changed at the terminal boundary.

## Core subjects

### Geometry

Geometry is represented as typed values: `Point`, `Size`, `Rect`, and later regions, anchors, insets, and hit areas. Rectangles carry spatial meaning independent of terminal escape sequences. Coordinate conventions, integer ranges, clipping, and empty rectangles must be explicit and testable.

### Constraints

Layout consumes a widget tree plus constraints and produces placements. A constraint can describe fixed, intrinsic, minimum, maximum, proportional, or grow/shrink behavior. Resolution must have a bounded failure surface for conflicting constraints, overflow, and insufficient terminal space.

The solver should return structured results, including unresolved obligations or overflow details where applicable. It must not silently turn an unsatisfied constraint into an arbitrary placement.

### Widget graph

Widgets are semantic nodes, not only render callbacks. A node has an identity, parent/child relationships, layout intent, focus participation, event behavior, and rendering behavior. Composition should allow containers, panes, lists, tables, forms, overlays, and status regions to be assembled without each widget inventing a lifecycle protocol.

### Events

The event boundary normalizes terminal input into typed events: keys, mouse actions, paste, resize, focus, and lifecycle events. Raw bytes and terminal escape sequences belong to the adapter boundary. Applications should observe events with explicit source, timing, and capability assumptions where those facts matter.

Event handling is a state transition. It should expose handled/unhandled outcomes, follow-up effects, focus changes, invalidated regions, and cleanup obligations rather than hiding them in a callback convention.

### Frames and diffs

Widgets produce structured render intent. The frame model should retain cell coordinates, glyph or content identity, style, layer/order, clipping, and ownership. The diff stage compares the current frame to a prior frame and emits a bounded damage set or a full redraw decision with reasons.

The terminal projection is deliberately last. It is responsible for capability-specific encoding, cursor movement, mode changes, and flushing. It must not be the place where layout or widget semantics are reconstructed.

## Ownership boundaries

```text
mncs-tui
  owns framework vocabulary, composition, layout policy, frame/diff policy,
  terminal adapters, and TUI-specific contracts

mncs-language
  owns syntax, type/contract/effect semantics, identities, verification,
  compiler/lowering, and backend contracts

mncs-language-service
  owns resident workspace analysis, diagnostics, navigation, context, and
  editor/agent protocol adaptation

terminal adapter / host
  owns OS-specific input and output transport, terminal capability probing,
  and resource acquisition at the external boundary
```

The framework may define TUI-specific concepts, but it should not reimplement language validation, evidence, authority, or source identity. If a concept cannot be expressed safely in MNCS, record that as upstream language pressure.

## State and effect boundaries

The intended update cycle is:

1. receive a bounded batch of normalized events;
2. apply event transitions to application/widget state;
3. resolve layout if state or terminal size invalidated it;
4. render a structured frame for the resolved geometry;
5. diff against the previous accepted frame;
6. project the diff through the terminal adapter;
7. commit the new frame only when the projection boundary reports its result.

Terminal mode changes, input reads, output writes, timers, and resize observation are effects. Their authority, failure behavior, and cleanup obligations should remain visible to the language and its service rather than being implicit global state.

## Verification pressure

The first useful verification targets are bounded and local:

- child rectangles remain within their parent clipping region;
- siblings do not overlap unless their layer policy allows it;
- fixed and minimum constraints are preserved when a solution exists;
- impossible constraints produce a structured unresolved result;
- frame diffs preserve unchanged cells and cover every changed cell;
- event dispatch respects focus and hit-test relationships;
- terminal mode acquisition is paired with restoration on normal and failure paths;
- a resize invalidates the correct layout/frame subjects.

These are candidate obligations, not claims that the current scaffold proves them.

## Open design questions

- Which geometry and collection types belong in the MNCS standard library versus this framework?
- What is the right bounded solver representation for agent inspection and compiler lowering?
- How should wide glyphs, combining marks, and ambiguous terminal cell widths be modeled?
- Should frame identity be cell-based, region-based, or a layered hybrid?
- Which terminal capability facts are observations, assumptions, or evidence?
- What is the smallest MNCS-language-service context packet that lets an agent repair a layout without losing semantic relationships?

These questions should become focused experiments and upstream issues, not accidental API commitments.
