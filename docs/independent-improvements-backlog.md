# Independent Improvements Backlog (Post Week 2)

## Purpose
Track independent improvements that are useful but not hard blockers for Week 2 execution.

## Priority A (High Value, Decoupled)
1. Conformance schema runtime validation (`jsonschema` crate)
- Goal: validate every case file against `tests/conformance/_schema/ccml-conformance-case.schema.json` before execution.
- Why later: runner decoupling to `serde_json` is the urgent fix; schema validation can land as a follow-up.
- Done when:
  - runner fails fast on schema-invalid case files with clear error class (`schema`).
  - valid case files keep existing pass/fail behavior.
- Validation:
  - `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`

2. Conformance vector count expansion
- Goal: increase vectors from 46 to >= 60.
- Focus classes:
  - control/unicode escape edges
  - mixed delimiter/layout corners
  - diagnostic boundary positions
- Done when:
  - `tests/conformance/INDEX.md` reflects updated totals and classes.
  - runner remains green for updated suite.

## Priority B (Developer Experience / Maintainability)
3. `thiserror` integration for internal error typing
- Goal: reduce manual error mapping boilerplate while keeping external error contract stable.
- Guardrail: public FFI error JSON envelope must not drift.
- Done when:
  - internal errors are typed and mapped consistently.
  - FFI contract tests remain green.

4. `clap` integration for `ccml-cli`
- Goal: standardize CLI parsing and prepare flags/subcommands growth.
- Guardrail: existing behavior (`conformance` mode and stdin path) must remain backward-compatible.
- Done when:
  - existing commands behave identically.
  - help/usage output is deterministic and tested.

## Explicit Non-Goal
- Do not replace `ccml-core` AST->JSON serializer with `serde_json`.
- `serde_json` remains limited to conformance case file loading/validation tooling.
