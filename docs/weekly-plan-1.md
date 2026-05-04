# Weekly Plan 1 (2026-04-28 to 2026-05-04)

## 1. Weekly Goal
1. Rust core implementation kickoff with stable module boundaries.
2. Conformance vectors expansion focused on edge cases.
3. Detailed execution structure for next-step polyglot delivery.

## 2. Scope
- In scope:
  - Rust core skeleton and minimal parser behavior
  - Conformance test vectors (`valid`, `invalid`, `warn`) expansion
  - Diagnostics and warning policy alignment (`keep-last + warn`)
  - Plan/design detail updates for Python/Node/WASM integration
- Out of scope:
  - Full Python/Node binding implementation
  - Production release automation completion

## 3. Day-by-Day Plan

### Day 1 - Tue 2026-04-28
- Freeze Rust workspace and crate responsibilities:
  - `ccml-core`, `ccml-cli`, `ccml-ffi`, `ccml-wasm`
- Freeze core API signatures:
  - `parse(text)`, `to_json(text, options)`, `diagnose(text)`
- Deliverables:
  - updated architecture/service boundary notes
  - crate ownership table

### Day 2 - Wed 2026-04-29
- Expand conformance vectors phase 1 (30-50 target):
  - bare key accepted/disallowed patterns
  - comments and separators
  - implicit root object
  - numeric-looking and keyword-like keys
- Deliverables:
  - new vector files
  - vector index document

### Day 3 - Thu 2026-04-30
- Build minimal Rust parser path in `ccml-core`:
  - object, array, string, number, boolean, null
  - implicit root object behavior
- Deliverables:
  - minimal parse/transcode path
  - initial unit test pass

### Day 4 - Fri 2026-05-01
- Implement diagnostics baseline:
  - error code + line + column
  - duplicate key `keep-last + warn`
- Deliverables:
  - diagnostic output contract implementation
  - warning path tests

### Day 5 - Sat 2026-05-02
- Build conformance runner prototype for Rust:
  - consume shared vector schema
  - verify `valid`, `invalid`, `warn`
- Deliverables:
  - executable local runner
  - pass/fail summary output

### Day 6 - Sun 2026-05-03
- Refine polyglot integration plan:
  - FFI surface draft
  - Python/Node boundary contract
  - WASM packaging notes
- Deliverables:
  - updated polyglot implementation plan
  - binding TODO list

### Day 7 - Mon 2026-05-04
- Weekly validation and sprint-close:
  - conformance pass rate snapshot
  - known gaps and blockers
  - next-week priorities
- Deliverables:
  - weekly review summary
  - Sprint 2 backlog draft

## 4. Definition of Done (Week 1)
1. Rust core supports minimal parse + JSON transcode path.
2. Conformance vectors are expanded to at least 50 total cases.
3. Invalid-case diagnostics validate `code`, `line`, `column`.
4. Warn-case diagnostics validate duplicate-key warning behavior.
5. Polyglot boundary notes are concrete enough for binding kickoff.

## 5. Risks and Mitigations
1. Risk: parser implementation drifts from spec detail.
- Mitigation: conformance-first gating before feature merge.
2. Risk: diagnostics contract changes mid-week.
- Mitigation: freeze required fields (`code`, `line`, `column`) first.
3. Risk: vector quality over quantity imbalance.
- Mitigation: prioritize edge-case classes before bulk additions.

## 6. Priority Order
1. Correctness and compatibility with spec
2. Diagnostic clarity
3. Performance tuning (after correctness gate)

## 7. Execution Log
### Day 3 - Thu 2026-04-30 (Micro Loop 1)
- Plan:
  - Implement parser skeleton for core value/object/array flow with implicit root support.
- Design:
  - Split core into `ast.rs`, `diag.rs`, `parser.rs`, `json.rs`.
- Implement:
  - Added recursive-descent parser skeleton and JSON serializer path.
  - Wired public API: `parse`, `to_json`, `diagnose`.
- Validate:
  - `cargo test --offline -p ccml-core` passed (2/2 tests).
- Document:
  - Architecture and weekly plan kept in sync with implementation status.

### Day 3 - Thu 2026-04-30 (Micro Loop 2)
- Plan:
  - Add unicode escape parsing, tighten EOF/separator diagnostics, and expand conformance-linked unit tests.
- Design:
  - Keep parser contract stable and extend only parser internals (`parser.rs`) plus API-level tests (`lib.rs`).
- Implement:
  - Added `\\uXXXX` escape handling.
  - Added explicit missing-value error after `key:`.
  - Kept bare-key punctuation-only rejection in parser path.
  - Added unit tests for unicode escape, missing value, invalid bare key, and keyword-like key handling.
- Validate:
  - `cargo test --offline -p ccml-core` passed (6/6 tests).
- Document:
  - Day 3 loop-2 status recorded for traceability.

### Day 4 - Fri 2026-05-01
- Plan:
  - Implement diagnostics baseline and duplicate-key policy behavior.
- Design:
  - Keep parse API stable while adding internal diagnostics collection in parser.
- Implement:
  - Added parser-level diagnostics accumulator.
  - Implemented duplicate-key warning emission (`CCML2001`) with keep-last overwrite.
  - Updated `diagnose()` to return warnings even when parse succeeds.
- Validate:
  - `cargo test --offline -p ccml-core` passed (7/7 tests).
- Document:
  - Day 4 execution status recorded.

### Day 5 - Sat 2026-05-02
- Plan:
  - Build Rust conformance runner prototype using shared vector files.
- Design:
  - Add a `conformance` mode to `ccml-cli` so vectors can be run without extra external dependencies.
- Implement:
  - Added `ccml-cli conformance [path]` mode.
  - Runner reads all `valid/invalid/warn` vector files and validates status/output/warnings/errors.
  - Added semantic numeric equality in JSON comparison (e.g. `1e10` == `10000000000`).
  - Added BOM-tolerant whitespace handling in parser for UTF-8 BOM vector files.
- Validate:
  - `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
  - Result: `total=46`, `failed=5`.
- Document:
  - Remaining failures are now explicit and traceable for parser-diagnostic refinement.

### Day 6 - Sun 2026-05-03
- Plan:
  - Resolve parser diagnostic precision gaps and JSON transcode distortion risks first, then establish IR-to-native path with number raw preservation.
- Design:
  - Keep `parse -> IR(AST)` as source of truth.
  - Add explicit `IR -> NativeValue` conversion path.
  - Keep `IR -> JSON` path raw-preserving for number lexemes.
- Implement:
  - Improved parser diagnostic positioning and unexpected-token classification for trailing token scenarios.
  - Extended JSON string escaping for `\\b`, `\\f`, and all remaining control characters (`U+0000..U+001F`).
  - Added `native.rs` and `to_native()` API for explicit IR-to-native conversion.
  - Added native number type preserving raw lexeme with helper conversion methods.
  - Updated spec with number preservation policy for transcode/native modes.
- Validate:
  - `cargo test --offline -p ccml-core` passed (10/10 tests).
  - `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance` -> `total=46`, `failed=0`.
- Document:
  - Day 6 execution status recorded with mode-policy alignment complete.

### Day 7 - Mon 2026-05-04
- Plan:
  - Close Week 1 with objective validation and Sprint 2 preparation.
- Validate:
  - `cargo test --offline -p ccml-core` passed (10/10).
  - `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance` -> `total=46`, `failed=0`.
- Document:
  - Added week review summary with DoD check and known gaps.
  - Added Sprint 2 backlog draft with FFI-first priorities.
