# Examples

Examples are small MNCS source fixtures used to explore the framework vocabulary. They are not a stable public API and may need revision as MNCS source profiles evolve.

`first-layout.mncs` is intentionally modest: it constructs a typed rectangle from terminal dimensions. `full-demo.mncs` composes the current semantic pipeline. `focused-tests.mncs` contains bounded regression predicates for layout overflow and weighted allocation, damage clearing, widget-tree clipping/z-order, focus traversal, and upstream ANSI-event mapping. `backend-repro.mncs` is the minimized research-backend performance fixture documented in the roadmap.

The probe fixtures are semantic source experiments, not a host-terminal application. Build the CLI from the sibling `mncs-language` checkout and validate them with `MNCS_LIBRARY_PATH` set to both repositories.
