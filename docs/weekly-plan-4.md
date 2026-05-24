# Weekly Plan 4

## 1. Weekly Goal
1. Harden Node FFI runtime contract around ownership and repeated-call stability.
2. Improve CI gate visibility/reliability so failures are actionable without manual digging.
3. Promote WASM runtime smoke into a maintained quality signal with clear triage output.
4. Establish release-readiness baseline checks for Week 5 handoff.
5. Deepen conformance quality beyond count-focused expansion while preserving parser semantics.

## 2. Scope
- In scope:
  - Node FFI ownership hardening and regression coverage in `bindings/node`
  - CI gate artifact/timeout/path-trigger improvements
  - WASM smoke signal integration and diagnostics hardening
  - release-readiness checklist + version consistency checks
  - targeted conformance diagnostic-depth expansion
- Out of scope:
  - full public release execution (`v1.0.0` publishing/signing)
  - spec feature expansion beyond current CCML 1.0 draft behavior
  - broad refactor across unrelated crates/modules

## 3. Non-Negotiable Constraints for Week 4
1. `ccml-core` remains the single semantic source of truth.
2. No binding-specific parsing logic is allowed in Python/Node/WASM layers.
3. FFI ownership must remain explicit and single-path at boundary contracts.
4. CI improvements must increase signal quality without hiding failing states.
5. Conformance deepening must not relax existing diagnostics contracts (`code/line/column`).

## 4. Day-by-Day Plan

### Day 1
- Plan/Design:
  - Freeze Node memory-contract hardening targets from Week 3 residual risk.
  - Define regression-case matrix for repeated calls and mixed success/error paths.
- Implement:
  - Add/adjust Node-side wrapper behavior and tests for ownership stability.
- Validate:
  - Node smoke + focused Node regression suite pass.
- Deliverables:
  - Node memory-contract hardening baseline
  - Node regression test matrix (v1)

### Day 2
- Implement:
  - Extend CI summary artifact with per-gate failure pointers and log section references.
  - Add deterministic timeout guards to hanging-risk gate steps.
- Validate:
  - CI gate summary remains generated on pass/fail.
  - Timeout policy behavior is deterministic in workflow definition.
- Deliverables:
  - improved `artifacts/ci/gate-summary.md` contract
  - timeout policy baseline in CI workflows/scripts

### Day 3
- Implement:
  - Tighten CI path-scoped triggers and concurrency cancellation behavior.
  - Ensure gate set reflects active ownership paths only.
- Validate:
  - workflow trigger conditions map cleanly to touched paths.
  - concurrency cancellation behavior is explicit and stable.
- Deliverables:
  - updated CI trigger/concurrency policy
  - trigger-scope verification notes

### Day 4
- Implement:
  - Integrate WASM runtime smoke as a visible maintained gate signal.
  - Improve toolchain/version diagnostics and failure triage output for wasm path.
- Validate:
  - WASM smoke path reports actionable failure context.
  - shared parity fixture behavior checks remain green for success/error/warn.
- Deliverables:
  - hardened WASM smoke gate visibility
  - wasm triage diagnostics notes update

### Day 5
- Implement:
  - Draft release-readiness checklist across Rust/Python/Node/WASM surfaces.
  - Add baseline version-consistency checks across manifests/docs.
- Validate:
  - checklist is runnable and maps to concrete gate commands.
  - version-check logic catches intentional mismatch in dry validation.
- Deliverables:
  - release-readiness checklist draft
  - version consistency check baseline

### Day 6
- Implement:
  - Add focused conformance cases for invalid diagnostic boundaries and mixed stress patterns.
  - Keep expectations explicit and schema-valid.
- Validate:
  - conformance runner remains green with expanded quality-depth set.
  - warnings/diagnostics expectations remain deterministic.
- Deliverables:
  - new targeted conformance vectors
  - updated `tests/conformance/INDEX.md` summary

### Day 7
- Validate/Document:
  - Re-run week gate set and capture final pass snapshot.
  - Record residual risks and Week 5 release handoff blockers.
- Deliverables:
  - `docs/week4-review-summary.md`
  - `docs/sprint5-backlog-draft.md`

## 5. Definition of Done (Week 4)
1. Node memory-contract hardening is implemented with regression coverage.
2. CI artifacts provide clear, per-gate actionable failure pointers.
3. WASM runtime smoke is visible as a maintained gate signal with triage-friendly diagnostics.
4. Release-readiness checklist and baseline version consistency checks are in place.
5. Conformance quality depth is increased without regressing runner pass state.
6. Week 4 review and Sprint 5 backlog draft are documented.

## 6. Risks and Mitigations
1. Risk: Node FFI ownership behavior differs across runtime/library versions.
- Mitigation: keep regression suite focused on repeated/mixed call patterns and enforce stable failure semantics.

2. Risk: CI signal quality changes increase workflow complexity and flakiness.
- Mitigation: keep changes incremental, path-scoped, and validated with explicit artifact contracts.

3. Risk: WASM smoke failures are hard to triage due to toolchain drift.
- Mitigation: include version/tooling diagnostics directly in workflow and failure summaries.

4. Risk: Conformance deepening introduces unstable diagnostic expectations.
- Mitigation: add vectors in small batches and keep assertion boundaries explicit.

## 7. Priority Order
1. Node runtime contract hardening
2. CI gate visibility/reliability
3. WASM smoke signal hardening
4. Release-readiness baseline
5. Conformance quality deepening

## 8. Validation Commands (Target)
- `cargo test --offline -p ccml-core`
- `cargo test --offline -p ccml-ffi`
- `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
- `UV_CACHE_DIR=.uv-cache uv run python tests/smoke.py`
- `powershell -ExecutionPolicy Bypass -File tests/bindings/run-parity-smoke.ps1`
- `bash scripts/ci/run-ci-gates.sh`
- `bash scripts/wasm/build-and-run-smoke.sh`

## 9. Execution Log
### Day 1
- Plan:
  - Scoped Day 1 to Node runtime boundary hardening only (no parser/FFI contract expansion).
  - Locked first regression shape to repeated mixed call cycles (success -> error -> warn).
- Design:
  - Reuse shared parity fixture (`tests/bindings/parity-smoke-cases.json`) to keep cross-binding expectation parity.
  - Add one dedicated Node regression script to isolate runtime-sequence stability checks.
- Implement:
  - Added Node regression smoke:
    - `bindings/node/tests/day1-regression.mjs`
  - Added package script:
    - `bindings/node/package.json` -> `smoke:day1:regression`
  - Added Day 1 hardening target/matrix note:
    - `docs/node-memory-regression-matrix.md`
- Validate:
  - `node bindings/node/tests/day2-smoke.mjs`
  - `node bindings/node/tests/day1-regression.mjs`
- Document:
  - Recorded Day 1 execution details in this log.

### Day 2
- Plan:
  - Target Day 2 only on CI gate observability and deterministic timeout control.
  - Keep gate set unchanged; improve only execution guardrails and failure readability.
- Design:
  - Add per-gate timeout wrapper in `scripts/ci/run-ci-gates.sh` with explicit reason codes (`ok`, `timeout`, `exit(n)`).
  - Extend summary artifact with log pointers and failed-gate tail snippets.
  - Add workflow-level timeout guard in `.github/workflows/ci-gates.yml` for bounded run time.
- Implement:
  - Updated `scripts/ci/run-ci-gates.sh`:
    - per-gate timeout env defaults (`GATE_TIMEOUT_*_SECONDS`)
    - timeout-based gate execution wrapper
    - reason fields per gate
    - log path pointers in summary
    - failed-gate tail snippet section in summary
  - Updated `.github/workflows/ci-gates.yml`:
    - `jobs.gates.timeout-minutes: 35`
    - `Run CI gates` step timeout: 30 minutes
- Validate:
  - `bash -n scripts/ci/run-ci-gates.sh`
  - YAML field-level sanity check for workflow timeout keys
- Document:
  - Recorded Day 2 execution details in this log.

### Day 3
- Plan:
  - Limit CI triggers to paths that affect the current gate set (core/ffi/conformance/python/ci scripts).
  - Add explicit concurrency cancellation to prevent stale runs from consuming queue time.
- Design:
  - Use workflow-level `paths` + `paths-ignore` for both `push` and `pull_request`.
  - Keep manual execution available via `workflow_dispatch`.
  - Use one deterministic concurrency group keyed by workflow and ref.
- Implement:
  - Updated `.github/workflows/ci-gates.yml`:
    - Added `push.paths`/`pull_request.paths` scoped to:
      - `.github/workflows/ci-gates.yml`
      - `core/rust/**`
      - `bindings/python/**`
      - `tests/conformance/**`
      - `scripts/ci/**`
    - Added `paths-ignore` for `docs/**` and `old_temp/**`
    - Added `workflow_dispatch`
    - Added concurrency policy:
      - `group: ci-gates-${{ github.ref }}`
      - `cancel-in-progress: true`
- Validate:
  - Verified trigger and concurrency keys exist with expected values in workflow file.
  - Checked that Day 3 scope now matches active gate ownership paths only.
- Document:
  - Recorded Day 3 execution details in this log.

### Day 4
- Plan:
- Design:
- Implement:
- Validate:
- Document:

### Day 5
- Plan:
  - Start release-readiness baseline only; do not expand to publish/signing automation.
  - Add one deterministic version-consistency gate across Rust/Python/Node manifests.
- Design:
  - Create a checklist doc with executable gates and RC blocker draft.
  - Implement a lightweight CI helper script to compare manifest versions:
    - Rust: `core/rust/Cargo.toml` (`[workspace.package].version`)
    - Python: `bindings/python/pyproject.toml` (`[project].version`)
    - Node: `bindings/node/package.json` (`version`)
- Implement:
  - Added release-readiness checklist:
    - `docs/release-readiness-checklist.md`
  - Added version consistency checker:
    - `scripts/ci/check-version-consistency.sh`
- Validate:
  - Manual source-value check confirms current versions align at `0.1.0`:
    - `core/rust/Cargo.toml`
    - `bindings/python/pyproject.toml`
    - `bindings/node/package.json`
  - Script execution is intended via bash environments (CI/Linux path).
- Document:
  - Recorded Day 5 execution details in this log.

### Day 6
- Plan:
  - Expand conformance with focused invalid diagnostic-boundary cases.
  - Add mixed stress warn cases combining duplicate-key policy with escapes/comments/layout.
- Design:
  - Keep schema shape unchanged and append numbered cases only.
  - Add 2 `invalid` + 2 `warn` cases to raise quality depth without widening feature scope.
- Implement:
  - Added invalid diagnostic-boundary cases:
    - `tests/conformance/invalid/012-missing-colon-after-newline-key.json`
    - `tests/conformance/invalid/013-missing-value-before-eof-newline.json`
  - Added mixed stress warn cases:
    - `tests/conformance/warn/014-duplicate-key-escaped-string-comment.json`
    - `tests/conformance/warn/015-duplicate-key-array-to-object-escape.json`
  - Updated conformance index summary/classes:
    - `tests/conformance/INDEX.md`
- Validate:
  - `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
  - Result: `total=64`, `failed=0`
  - Note: one initial location mismatch on new invalid case `013` was corrected to observed parser location (`3:1`), then rerun green.
- Document:
  - Recorded Day 6 execution details in this log.

### Day 7
- Plan:
- Design:
- Implement:
- Validate:
- Document:
