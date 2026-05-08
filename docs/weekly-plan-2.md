# Weekly Plan 2 (2026-05-05 to 2026-05-11)

## 1. Weekly Goal
1. Complete `ccml-ffi` phase-1 C ABI implementation.
2. Establish FFI contract tests and conformance smoke path.
3. Start Python/Node binding kickoff with explicit boundary contracts.
4. Harden conformance runner input path without changing core AST->JSON behavior.
5. Lock Week 2 interop toolchain decisions (`cbindgen`, `wasm-bindgen`).

## 2. Scope
- In scope:
  - JSON-only FFI surface (`ccml_to_json`, `ccml_diagnose`, `ccml_version`, `ccml_free`)
  - return-code + error-json contract
  - alloc/free ownership contract and null/UTF-8 guards
  - conformance smoke checks through FFI path
  - conformance case loading hardening (`serde`/`serde_json` for case loading)
  - `cbindgen` integration for C header generation from FFI surface
  - `wasm-bindgen` integration baseline for `ccml-wasm`
- Out of scope:
  - full Node binding implementation
  - full release automation completion
  - v1.0.0 release tasks
  - any replacement of core parser or core JSON serializer

## 3. Non-Negotiable Constraints for Week 2
1. `ccml-core` remains the single semantic source of truth.
2. `AST -> JSON` output path must stay custom in `ccml-core`; do not use `serde_json` for core output generation.
3. `serde_json` usage is allowed only for conformance case file loading/validation path.
4. Every FFI-returned heap buffer must be releasable only via `ccml_free`.

## 4. Day-by-Day Plan

### Day 1 - Tue 2026-05-05
- Freeze FFI phase-1 interface:
  - function signatures
  - return code set
  - memory ownership rules
- Decide error-json envelope fields and versioning policy.
- Integrate `cbindgen` config and generated header workflow draft.
- Deliverables:
  - FFI interface draft document
  - `ccml-ffi` function skeleton

### Day 2 - Wed 2026-05-06
- Implement `ccml_to_json` and `ccml_diagnose`.
- Implement argument validation (null pointer, UTF-8 invalid input).
- Add panic guard and internal error mapping baseline.
- Deliverables:
  - working FFI transcode/diagnose path
  - error-json return path

### Day 3 - Thu 2026-05-07
- Implement `ccml_version` and `ccml_free`.
- Finalize memory ownership contract in code/tests.
- Set `wasm-bindgen` baseline wiring in `ccml-wasm`.
- Deliverables:
  - complete phase-1 API surface
  - alloc/free contract enforcement

### Day 4 - Fri 2026-05-08
- Add FFI contract tests:
  - success/error/warn path
  - pointer/argument safety
  - invalid UTF-8 handling
  - free-after-return behavior
- Deliverables:
  - FFI contract test suite
  - failure-case verification logs

### Day 5 - Sat 2026-05-09
- Harden conformance runner input path:
  - parse conformance case files via `serde`/`serde_json`
  - keep `ccml-core` parse/to_json/diagnose as execution target for `input`
- Deliverables:
  - serde-based conformance loading path
  - runner failure messages separated into schema/load/runtime categories

### Day 6 - Sun 2026-05-10
- Run conformance smoke through FFI path using representative `valid/invalid/warn` vectors.
- Start Python thin adapter kickoff.
- Add minimal usage tests for:
  - to_json path
  - diagnose path
  - duplicate warning propagation (`CCML2001`)
- Deliverables:
  - FFI smoke report with parity notes
  - Python adapter prototype and smoke results

### Day 7 - Mon 2026-05-11
- Weekly validation and sprint close:
  - pass-rate snapshot
  - known gaps/blockers
  - Week 3 priorities
- Deliverables:
  - week2 review summary
  - sprint3 backlog draft

## 5. Definition of Done (Week 2)
1. `ccml-ffi` phase-1 API is implemented and callable.
2. FFI contract tests pass for success/error/warn and memory safety basics.
3. Conformance smoke checks pass through FFI path for selected fixtures.
4. Conformance loader uses `serde_json` (case files only) and no longer depends on `ccml-core::parse` for vector file parsing.
5. Core `AST -> JSON` output behavior remains unchanged and still preserves number raw lexemes policy.
6. Python thin adapter prototype successfully calls FFI happy/error paths.
7. `cbindgen` header generation path and `wasm-bindgen` baseline are both executable.

## 6. Risks and Mitigations
1. Risk: FFI memory ownership misuse.
- Mitigation: enforce single free path (`ccml_free`) and test pointer lifecycle explicitly.

2. Risk: ambiguous failure source in conformance runs.
- Mitigation: split failure classes into (a) case schema/load, (b) ccml input runtime behavior.

3. Risk: accidental serializer drift by introducing `serde_json` in core output path.
- Mitigation: keep serializer boundary explicit in code review; add regression checks for escape/number raw behavior.

4. Risk: binding contract drift across runtimes.
- Mitigation: freeze return-code + error-json envelope before adapter expansion.

## 7. Priority Order
1. ABI and memory safety
2. Diagnostic/error contract stability
3. Conformance runner hardening (loader/schema)
4. `cbindgen`/`wasm-bindgen` interop baseline
5. Binding usability kickoff
6. Conformance case count expansion toward Week 3

## 8. Validation Commands (Target)
- `cargo test --offline -p ccml-core`
- `cargo test --offline -p ccml-ffi`
- `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
- FFI smoke command/script (to be finalized in Week 2 Day 5)

## 9. Execution Log
### Day 2 - Wed 2026-05-06
- Plan:
  - Implement `ccml_to_json` and `ccml_diagnose` using `ccml-core` APIs.
  - Add null/UTF-8 argument validation and panic-to-error mapping baseline.
- Design:
  - Keep JSON-only C ABI with out-parameters and status-code return.
  - Keep `ccml_free` as single deallocation path.
- Implement:
  - Wired `ccml_to_json` to `ccml_core::to_json` (compact mode).
  - Wired `ccml_diagnose` to `ccml_core::diagnose`.
  - Added UTF-8 decode path from `(input_ptr, input_len)`.
  - Added `catch_unwind` guards for both FFI entrypoints.
  - Added baseline error JSON payload with optional diagnostics array on parse errors.
- Validate:
  - `cargo test --offline -p ccml-ffi` passed.
  - `cargo test --offline -p ccml-core` passed (10/10).
- Document:
  - Day 2 execution status recorded in this log.

### Day 3 - Thu 2026-05-07
- Plan:
  - Finalize `ccml_version`/`ccml_free` contract confidence with tests.
  - Add `wasm-bindgen` baseline wiring in `ccml-wasm`.
- Design:
  - Keep FFI memory contract explicit and test caller-release behavior.
  - Expose minimal WASM entrypoints mirroring core behavior (`to_json`, `diagnose`).
- Implement:
  - Added FFI tests for version allocation and null-safe free handling.
  - Added `wasm-bindgen` dependency and baseline exported functions in `ccml-wasm`.
  - Wired WASM paths to `ccml-core` while keeping core as single source of truth.
- Validate:
  - `cargo test --offline -p ccml-ffi` passed (2/2 tests).
  - `cargo test -p ccml-wasm` passed after dependency fetch.
- Document:
  - Day 3 execution status recorded in this log.

### Day 4 - Fri 2026-05-08
- Plan:
  - Expand FFI contract tests for success/error/warn and pointer-safety behavior.
- Design:
  - Keep tests at ABI boundary (`ccml_to_json`, `ccml_diagnose`, `ccml_version`, `ccml_free`).
  - Validate status code mapping and out-pointer ownership behavior together.
- Implement:
  - Added success-path test for `ccml_to_json`.
  - Added parse-error and invalid-UTF8 tests for `ccml_to_json` error contract.
  - Added invalid-argument pointer validation tests.
  - Added warn-path test for `ccml_diagnose` duplicate-key warning propagation.
  - Kept `ccml_version`/`ccml_free` contract tests from Day 3.
- Validate:
  - `cargo test --offline -p ccml-ffi` passed (8/8 tests).
- Document:
  - Day 4 execution status recorded in this log.
