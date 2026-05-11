# Sprint 3 Backlog Draft

## Priority 1 - Node Binding Kickoff
1. Create `bindings/node` skeleton and load `ccml-ffi` dynamic library.
2. Implement minimal adapter for:
- `to_json`
- `diagnose`
- memory release path
3. Add Node smoke tests for success/error/warn parity with Python adapter.

## Priority 2 - CI Gating Foundation
1. Add CI jobs for:
- `cargo test --offline -p ccml-core`
- `cargo test --offline -p ccml-ffi`
- `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
- Python smoke (`uv run python tests/smoke.py`)
2. Publish pass/fail summary artifact per run.

## Priority 3 - WASM Runtime Verification
1. Add runtime smoke harness for `ccml-wasm` exports in JS runtime.
2. Verify:
- transcode success path
- error payload propagation
- diagnose warning visibility

## Sprint 3 Exit Criteria (Draft)
1. Node adapter prototype reaches same smoke coverage level as Python adapter.
2. CI executes core/ffi/conformance/python smoke on every change.
3. WASM runtime smoke passes for baseline exports.
