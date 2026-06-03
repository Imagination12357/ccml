# Week 5 Release Go/No-Go Snapshot

## Current Decision
No-Go for immediate `v1.0.0` publishing until the blockers below are closed.

## Local Gate Snapshot
1. Core tests: pass (`11 passed, 0 failed`)
2. FFI tests: pass (`9 passed, 0 failed`)
3. Conformance: pass (`total=68, failed=0`)
4. Python/Node parity smoke: pass
5. Node Day 1 regression: pass (`300 mixed cycles + large payload + invalid variants`)
6. Python direct smoke with repo-local uv cache: pass
7. Windows version consistency fallback: pass (`0.1.0` aligned across Rust/Python/Node)
8. Root README baseline: present.

## Blocker Ledger
1. Release version/tag is not finalized against manifests.
- Current manifests are aligned at `0.1.0`.
- Planned release target is `v1.0.0`.
- Next trigger: decide whether to bump manifests to `1.0.0` or change release target.

2. CI-hosted `ci-gates` evidence is not captured in this local snapshot.
- Local runnable gates are green.
- Next trigger: latest target-branch `ci-gates` run is green and gate summary artifact is available.

3. CI-hosted WASM smoke evidence is not captured in this local snapshot.
- Local Windows environment lacks `bash`, so the build/package script cannot run locally.
- Next trigger: latest target-branch `wasm-runtime-smoke` run is green and `wasm-smoke-summary` reports `status: pass`.

4. Node publish metadata is unresolved.
- `bindings/node/package.json` currently has `private: true`.
- Next trigger: decide whether Node is release-scoped for public publish or explicitly excluded from the first release.

5. README release polish is newly added and needs final owner review.
- Root README now exists and documents status, layout, CLI usage, validation, bindings, and release readiness.
- Next trigger: final README review before release tag.

## Go Criteria
Mark Go only when:
1. Version/tag decision is finalized.
2. Mandatory local gates remain green.
3. `ci-gates` is green on the target branch.
4. `wasm-runtime-smoke` is green on the target branch.
5. `wasm-smoke-summary` reports `status: pass`.
6. Node publish scope is explicitly decided.
7. README release polish is accepted.

## No-Go Closeout Criteria
If any blocker remains on Day 7:
1. publish a release-prep closeout instead of executing release.
2. keep this blocker ledger as the next release trigger list.
