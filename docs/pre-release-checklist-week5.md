# Pre-Release Checklist (Week 5 Final)

## Purpose
Final go/no-go checklist before release execution (or release-ready closeout) in Week 5.

## 1) Version and Metadata Freeze
1. Manifest versions are aligned:
- `core/rust/Cargo.toml` (`[workspace.package].version`)
- `bindings/python/pyproject.toml` (`[project].version`)
- `bindings/node/package.json` (`version`)
2. Version consistency checker passes:
- `bash scripts/ci/check-version-consistency.sh`
3. Release version and tag target are explicitly fixed in release notes draft.

## 2) Mandatory Quality Gates
1. `cargo test --offline -p ccml-core`
2. `cargo test --offline -p ccml-ffi`
3. `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
4. `powershell -ExecutionPolicy Bypass -File tests/bindings/run-parity-smoke.ps1`
5. `node bindings/node/tests/day1-regression.mjs`
6. WASM runtime smoke path is green in CI signal set.
7. WASM smoke summary artifact reports `status: pass`.

## 3) CI Signal Health
1. `ci-gates` workflow latest run is green on target branch.
2. Gate summary artifact contains:
- per-gate status
- per-gate reason
- log pointers
- failed-gate snippet section (empty/irrelevant on green runs)
3. No unresolved flaky failures in the most recent release-candidate runs.
4. WASM smoke summary artifact is available as `wasm-smoke-summary`.

## 4) Runtime Contract Confidence
1. Node memory ownership risk status is explicitly recorded (closed or known residual with acceptance).
2. Python/Node parity expectations remain aligned on shared fixture.
3. No known error-envelope drift across bindings for status/error semantics.

## 5) Documentation and Release Artifacts
1. Week 5 review summary is drafted.
2. Release notes include:
- scope summary
- known limitations
- validation snapshot
3. Release runbook contains publish order and rollback/fallback notes.

## 6) Go / No-Go Decision
1. Go when all mandatory gates are green and no blocker remains.
2. No-Go when any blocker exists:
- failing mandatory gate
- unresolved version mismatch
- unresolved release-critical runtime contract issue
3. If No-Go:
- publish a release-prep closeout with explicit blockers and next trigger conditions.
