# Week 2 Review Summary (2026-05-05 to 2026-05-11)

## 1) Outcome Snapshot
- `ccml-ffi` phase-1 C ABI baseline implemented and callable.
- FFI contract tests expanded (success/error/warn/argument safety).
- FFI-path conformance smoke added for representative fixtures.
- Python thin adapter kickoff completed with `uv` workflow and smoke tests.
- `ccml-wasm` baseline wired with `wasm-bindgen`.

## 2) Validation Results
- `ccml-core` tests: 10 passed, 0 failed.
- `ccml-ffi` tests: 9 passed, 0 failed.
- `ccml-wasm` tests: 0 passed, 0 failed (compile-level baseline verified).
- Conformance runner (`ccml-cli`): `total=46`, `failed=0`.
- Python smoke (`uv run python tests/smoke.py`): passed.

## 3) Week 2 DoD Check
1. `ccml-ffi` phase-1 API implemented and callable: met.
2. FFI contract tests pass for success/error/warn and memory safety basics: met.
3. Conformance smoke checks pass through FFI path for selected fixtures: met.
4. Conformance loader uses `serde_json` for case files and no longer depends on `ccml-core::parse` for vector file parsing: met.
5. Core `AST -> JSON` output behavior unchanged and raw-number policy preserved: met.
6. Python thin adapter prototype calls succeed for happy/error/warn paths: met.
7. `cbindgen` header generation path and `wasm-bindgen` baseline executable: met (header generation config present, wasm baseline compiled).

## 4) Known Gaps / Blockers
- FFI contract tests are strong but still local Rust test scope only; no cross-language CI gate yet.
- Conformance runtime schema validation (`jsonschema`) is still deferred.
- Node binding implementation has not started.
- Conformance vector count remains 46 (target expansion to >= 60 deferred).
- WASM runtime smoke (actual JS runtime invocation) is not yet added.

## 5) Recommended Week 3 Priorities
1. Start Node binding skeleton and parity smoke with Python adapter behavior.
2. Add CI jobs for core/ffi/conformance/python smoke.
3. Add conformance schema runtime validation in CLI runner.
4. Expand conformance vectors to >= 60 focused edge classes.
5. Add WASM runtime smoke path (Node/browser harness) for exported functions.
