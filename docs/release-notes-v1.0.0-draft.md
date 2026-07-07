# CCML v1.0.0 Release Notes Draft

## Status
Draft for Week 5 release decision. Do not publish until the go/no-go checklist is marked Go.

## Scope
CCML v1.0.0 is the planned production-ready release track for the Rust-core rebuild.

Included surfaces:
1. Rust core parser and JSON/native conversion path
2. `ccml-cli` conformance runner path
3. `ccml-ffi` C ABI boundary
4. Python thin adapter over `ccml-ffi`
5. Node thin adapter over `ccml-ffi`

Release-readiness signals, not `v1.0.0` package publish targets:
1. WASM runtime smoke path
2. `artifacts/ci/wasm-smoke-summary.md`

Deferred to post-`1.0.0`:
1. `ccml-wasm` package publishing
2. macOS prebuilt packages

## Highlights
1. Rust core remains the single semantic source of truth.
2. Conformance suite covers valid, invalid, and warning behavior with machine-readable fixtures.
3. Duplicate-key policy is keep-last with `CCML2001` warnings.
4. FFI ownership is explicit: Rust allocates returned strings, callers release through `ccml_free`.
5. Python and Node adapters share parity smoke fixtures.
6. WASM smoke produces a release-readiness summary artifact without expanding the `v1.0.0` publish scope.

## Validation Snapshot
Current local snapshot:
1. `cargo test --offline -p ccml-core` -> 11 passed, 0 failed.
2. `cargo test --offline -p ccml-ffi` -> 9 passed, 0 failed.
3. `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance` -> total=68, failed=0.
4. `powershell -ExecutionPolicy Bypass -File tests/bindings/run-parity-smoke.ps1` -> passed.
5. `node bindings/node/tests/day1-regression.mjs` -> passed (`300 mixed cycles + large payload + invalid variants`).
6. `powershell -ExecutionPolicy Bypass -File scripts/ci/check-version-consistency.ps1` -> Rust/Python/Node all `1.0.0`.
7. `Set-Location bindings/python; $env:UV_CACHE_DIR='.uv-cache'; uv run python tests/smoke.py` -> passed.

CI-hosted validation:
1. `ci-gates` latest target-branch run -> green (user-provided evidence).
2. `wasm-runtime-smoke` latest target-branch run -> green (user-provided evidence).
3. `artifacts/ci/wasm-smoke-summary.md` -> `status: pass` (user-provided evidence).

## Known Limitations
1. Local Windows environment does not provide `bash`, so bash-only CI scripts and WASM package build are CI/Linux validated.
2. Publish/tag execution has not been run from this local session.
3. `v1.0.0` prebuilt binary package scope is Windows x64 and Linux x64.
4. WASM package publishing and macOS prebuilt packages are post-`1.0.0` work.

## Release Decision
`docs/release-go-no-go-week5.md` is marked Go-ready. Publish/tag execution still requires an explicit operator action.
