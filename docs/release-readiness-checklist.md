# Release Readiness Checklist (Draft)

## Scope
Minimal Week 4 baseline for release-readiness preparation before Week 5 handoff.

## 1) Version Consistency
1. Rust workspace version (`core/rust/Cargo.toml`) is aligned with:
- `bindings/python/pyproject.toml`
- `bindings/node/package.json`
2. Version consistency check script passes:
- `bash scripts/ci/check-version-consistency.sh`

## 2) Core Quality Gates
1. Core tests:
- `cargo test --offline -p ccml-core`
2. FFI tests:
- `cargo test --offline -p ccml-ffi`
3. Conformance:
- `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
4. Python smoke:
- `UV_CACHE_DIR=.uv-cache uv run python tests/smoke.py`

## 3) Cross-Binding Runtime Baseline
1. Python/Node parity smoke:
- `powershell -ExecutionPolicy Bypass -File tests/bindings/run-parity-smoke.ps1`
2. Node regression smoke:
- `node bindings/node/tests/day1-regression.mjs`
3. WASM runtime smoke path:
- `bash scripts/wasm/build-and-run-smoke.sh`
4. WASM smoke summary artifact reports `status: pass`:
- `artifacts/ci/wasm-smoke-summary.md`

## 4) CI Gate Visibility
1. `ci-gates` workflow uploads `artifacts/ci/gate-summary.md`.
2. Summary includes:
- per-gate status
- per-gate reason
- gate log path pointers
- failed-gate tail snippets

## 5) RC Candidate Blockers (Draft)
1. Any core/ffi/conformance gate failure.
2. Version mismatch across Rust/Python/Node package manifests.
3. Cross-binding smoke regression (Python/Node parity or Node Day 1 regression).
4. WASM runtime smoke not reproducible in CI path.
5. WASM smoke summary missing or reporting `status: fail`.

## Notes
1. This checklist is a release-readiness baseline, not full release automation.
2. Publishing/signing/release-note automation remains out of Week 4 scope.
