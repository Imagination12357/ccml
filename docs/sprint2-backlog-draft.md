# Sprint 2 Backlog Draft

## Priority 1 — FFI Phase-1 Implementation
1. Implement `ccml-ffi` C ABI surface:
- `ccml_to_json`
- `ccml_diagnose`
- `ccml_version`
- `ccml_free`
2. Implement return-code + error-json contract.
3. Implement alloc/free memory contract and null/UTF-8 guardrails.

## Priority 2 — FFI Quality Gates
1. Add FFI unit tests for success/error/diagnostic paths.
2. Add tests for pointer safety and invalid argument handling.
3. Add FFI smoke run over representative conformance fixtures (`valid`, `invalid`, `warn`).

## Priority 3 — Binding Kickoff
1. Build Python thin adapter over `ccml-ffi`.
2. Add Python smoke tests for:
- to_json path
- diagnose path
- duplicate warning propagation (`CCML2001`)
3. Define Node binding skeleton and call path parity checklist.

## Priority 4 — Conformance Expansion
1. Expand vectors from 46 to >= 60.
2. Focus additional cases:
- control/unicode escape edge cases
- mixed delimiter corner cases
- diagnostic boundary positions

## Priority 5 — Release/Automation Foundation
1. Add CI jobs for:
- core tests
- conformance runner
- FFI tests
2. Draft packaging checklist for crates/Python/Node artifacts.

## Sprint 2 Exit Criteria (Draft)
1. `ccml-ffi` phase-1 API implemented and tested.
2. FFI smoke checks pass against selected conformance vectors.
3. Python binding prototype calls succeed for core happy/error paths.
4. Conformance vectors >= 60 and runner remains green.
