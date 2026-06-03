# CCML v1.0.0 Release Runbook Draft

## Purpose
Define the release execution order and fallback actions for the planned `v1.0.0` release.

## Preconditions
1. `docs/release-go-no-go-week5.md` is marked Go.
2. Target version/tag is fixed.
3. Rust/Python/Node manifests are aligned to the release target.
4. `ci-gates` is green on the target branch.
5. `wasm-runtime-smoke` is green on the target branch.
6. WASM summary artifact reports `status: pass`.

## Validation Order
1. Version consistency:
`bash scripts/ci/check-version-consistency.sh`
2. Rust core:
`cd core/rust && cargo test --offline -p ccml-core`
3. FFI:
`cd core/rust && cargo test --offline -p ccml-ffi`
4. Conformance:
`cd core/rust && cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
5. Python smoke:
`cd bindings/python && UV_CACHE_DIR=.uv-cache uv run python tests/smoke.py`
6. Node parity/regression:
`node bindings/node/tests/day2-smoke.mjs`
`node bindings/node/tests/day1-regression.mjs`
7. WASM smoke:
`bash scripts/wasm/build-and-run-smoke.sh`

## Publish Order
1. Rust crates:
- `ccml-core`
- `ccml-cli`
- `ccml-ffi`
- `ccml-wasm`
2. Python package:
- `ccml-py`
3. Node package:
- `ccml-node`
4. Release tag:
- `v1.0.0`
5. GitHub release notes:
- use `docs/release-notes-v1.0.0-draft.md`

## Fallback / Rollback Notes
1. If validation fails before publishing:
- stop release.
- update blocker ledger.
- do not tag.
2. If Rust publish fails:
- stop downstream package publishing.
- keep tag uncreated until Rust surface is resolved.
3. If Python publish fails after Rust publish:
- stop Node publish.
- document partial release state and next trigger.
4. If Node publish is blocked by `private: true`:
- treat as release metadata blocker unless Node is explicitly scoped out.
5. If WASM smoke fails:
- do not publish `v1.0.0`.
- inspect `artifacts/ci/wasm-smoke-summary.md` first.

## Day 7 Decision Point
Day 7 decides whether this runbook is executed or whether the week ends with a release-prep closeout.
