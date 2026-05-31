# Independent Improvements Backlog (Post Week 2)

## Purpose
Track independent improvements that are useful but not hard blockers for Week 2 execution.

## Priority A (High Value, Decoupled)
1. [DONE 2026-05-05] Conformance schema runtime validation (`jsonschema` crate)
- Goal: validate every case file against `tests/conformance/_schema/ccml-conformance-case.schema.json` before execution.
- Why later: runner decoupling to `serde_json` is the urgent fix; schema validation can land as a follow-up.
- Done when:
  - runner fails fast on schema-invalid case files with clear error class (`schema`).
  - valid case files keep existing pass/fail behavior.
- Validation:
  - `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
- Implementation notes:
  - Added schema load/compile step in `ccml-cli` conformance path and per-case schema validation before case decode/execute.
  - Added explicit failure classification for schema path (`FAIL schema: ...`, `schema error: ...`).
  - Reduced `jsonschema` dependency surface to `default-features = false, features = ["draft202012"]`.
- Result:
  - `cargo test --offline -p ccml-cli` passed.
  - `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance` passed (`total=46, failed=0`).

2. Conformance vector count further expansion (post-Week 3)
- Goal: expand vectors beyond Week 3 baseline from >= 60 to >= 80.
- Focus classes:
  - control/unicode escape edges
  - mixed delimiter/layout corners
  - diagnostic boundary positions
  - cross-runtime parity-focused edge scenarios
- Done when:
  - `tests/conformance/INDEX.md` reflects updated totals and classes.
  - runner remains green for updated suite.

## Priority B (Developer Experience / Maintainability)
3. Cross-platform dynamic library name/path diagnostic improvement (Python/Node)
- Goal: improve failure message quality when FFI dynamic library cannot be found.
- Why independent: adapter UX improvement only; no runtime contract change.
- Done when:
  - error output includes searched paths and override env var guidance.
  - smoke tests still pass with explicit `CCML_FFI_LIB` override path.

4. Binding error-message wording consistency pass
- Goal: align Python/Node "library not found" and "install dependency" error wording for easier triage.
- Why independent: message quality only; status codes and payload contract unchanged.
- Done when:
  - top-level guidance mentions override env vars consistently.
  - smoke tests still pass with explicit override paths.

5. CI gate summary markdown readability polish
- Goal: improve `artifacts/ci/gate-summary.md` readability (compact section headings, consistent key order).
- Why independent: presentation-layer improvement only; gate logic unchanged.
- Done when:
  - summary keeps current pass/fail semantics.
  - output is easier to scan manually in artifact viewer.

6. Conformance index generation helper
- Goal: provide a tiny helper to recompute `tests/conformance/INDEX.md` totals from filesystem state.
- Why independent: maintenance tooling only; runner/spec semantics unchanged.
- Done when:
  - helper prints deterministic totals by class (`valid/invalid/warn`).
  - manual index drift can be detected quickly during review.

7. Conformance case lint utility (metadata/style guard)
- Goal: add a lightweight check for fixture consistency (id uniqueness, status/category consistency, trailing whitespace/BOM policy).
- Why independent: fixture maintenance tool only; runner semantics unchanged.
- Done when:
  - lint script reports actionable file-level messages.
  - existing 46+ fixtures pass lint in repository default state.

8. Node smoke script CLI UX tidy-up
- Goal: unify `bindings/node/package.json` smoke script naming (`smoke:day2`, `smoke:day3`, parity alias) with one obvious default.
- Why independent: command ergonomics only; wrapper/runtime contract unchanged.
- Done when:
  - one canonical smoke command exists for developers.
  - existing smoke assertions stay unchanged.

9. FFI C header generation smoke check (`cbindgen`)
- Goal: make header generation reproducible and verified in local/CI utility path.
- Why independent: does not change parser/binding semantics; packaging hygiene only.
- Done when:
  - one command generates `ccml.h` deterministically from `ccml-ffi`.
  - generated header is validated in a smoke step (no manual edit required).

10. `thiserror` integration for internal error typing
- Goal: reduce manual error mapping boilerplate while keeping external error contract stable.
- Guardrail: public FFI error JSON envelope must not drift.
- Done when:
  - internal errors are typed and mapped consistently.
  - FFI contract tests remain green.

11. `clap` integration for `ccml-cli`
- Goal: standardize CLI parsing and prepare flags/subcommands growth.
- Guardrail: existing behavior (`conformance` mode and stdin path) must remain backward-compatible.
- Done when:
  - existing commands behave identically.
  - help/usage output is deterministic and tested.

## Priority C (Low Risk Cleanup)
12. [DONE 2026-05-31] Parser duplicate-key insertion micro-optimization
- Goal: reduce duplicate key handling overhead without changing `keep-last + warn` behavior.
- Why independent: parser internal performance cleanup only; syntax, AST, diagnostics contract unchanged.
- Implementation notes:
  - Replaced `BTreeMap::contains_key` + `insert` with one `BTreeMap::entry` lookup in `ccml-core` object entry insertion.
  - Preserved `CCML2001` warning emission and keep-last overwrite behavior.
- Result:
  - `cargo test --offline -p ccml-core` passed.
  - `cargo test --offline -p ccml-cli` passed.
  - `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance` passed.

13. Recovery/playbook doc for offline toolchain execution
- Goal: document canonical fallback commands for PATH/network mismatch (`cargo --offline`, `uv` usage, ffi build path).
- Why independent: operational documentation only.
- Done when:
  - one short runbook exists under `docs/`.
  - commands are copy-pastable and validated at least once.

14. WASM smoke artifact cleanup utility
- Goal: add optional cleanup helper for generated `core/rust/target/wasm-smoke` artifacts.
- Why independent: local workspace hygiene only; build/test semantics unchanged.
- Done when:
  - one documented cleanup command/script exists.
  - CI/runtime smoke behavior remains unchanged with or without cleanup use.

## Explicit Non-Goal
- Do not replace `ccml-core` AST->JSON serializer with `serde_json`.
- `serde_json` remains limited to conformance case file loading/validation tooling.
