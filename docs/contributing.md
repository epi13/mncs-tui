# Contributing

`mncs-tui` is a language-family experiment as well as a framework repository. Contributions should make the semantic boundary clearer, add a reproducible fixture, or provide evidence for a design choice.

## Before changing code or fixtures

1. Read [the architecture](architecture.md) and identify the affected boundary.
2. Check whether the request is really a framework concern or belongs in `mncs-language` or `mncs-language-service`.
3. Keep experiments bounded and preserve `PASS`, `FAIL`, and `UNKNOWN` distinctions.
4. State when a source fixture is aspirational because the current MNCS profile cannot yet express it.

## Change categories

- **Framework change:** widget composition, layout policy, frame/diff behavior, or terminal adapter behavior.
- **Language pressure:** syntax, types, effects, contracts, identity, bounded computation, or compiler/lowering support needed by the framework.
- **Service pressure:** diagnostics, navigation, context, candidate analysis, or editor/agent interaction needed to work with TUI source.
- **Experiment:** a fixture or measurement that informs one of the above without making an API claim.

When a change crosses repositories, open the upstream issue or pull request and link it from the `mncs-tui` change. The framework should consume the authoritative upstream capability after it lands; it should not duplicate it temporarily in a way that becomes permanent.

## Source conventions

- Keep MNCS fixtures small, deterministic, and named after the behavior they exercise.
- Prefer explicit records and relationships over stringly-typed snapshots.
- Put terminal escape-sequence details at the adapter boundary.
- Document any coordinate convention, width assumption, or capability observation next to the fixture that depends on it.
- Do not call a bounded experiment a proof or a production implementation.

## Verification expectations

For every change, run the checks that are available for the current language revision. At minimum:

- inspect the resulting tree and diff;
- validate parseable MNCS fixtures with the matching `mncs-language` checkout when possible;
- check documentation links and examples for internal consistency;
- record unsupported or unresolved checks instead of silently skipping them.

Future phases will add corpus, layout, frame-diff, terminal, and language-service tests. Their results should remain reproducible from repository-local inputs.
