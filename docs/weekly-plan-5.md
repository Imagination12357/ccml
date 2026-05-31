# Weekly Plan 5

## 1. Weekly Goal
1. Close Node FFI ownership risk from regression baseline to explicit, test-backed contract confidence.
2. Integrate release-readiness checks into regular CI gating and improve cross-platform reproducibility.
3. Keep WASM runtime smoke as a release-quality signal with clearer triage paths.
4. Expand conformance diagnostic depth without regressing deterministic expectations.
5. Finish Week 5 with one of two outcomes:
- Release execution (`v1.0.0`) if blocker gates are green.
- Release-prep closeout with explicit blockers and trigger conditions if release is not ready.

## 2. Scope
- In scope:
  - Node ownership-contract closure and regression reinforcement in `bindings/node`
  - CI integration of version consistency checks and signal quality hardening
  - cross-platform validation playbook for Windows/Linux/macOS command parity
  - WASM smoke triage clarity and release-readiness linkage
  - focused conformance depth expansion (invalid boundary + mixed stress)
  - release notes/runbook/go-no-go decision flow
- Out of scope:
  - CCML 1.0 feature-surface expansion beyond current draft spec
  - broad architecture refactors across unrelated crates/modules
  - nonessential DX cleanup not tied to release confidence

## 3. Non-Negotiable Constraints for Week 5
1. `ccml-core` remains the single semantic source of truth.
2. No binding-specific parsing semantics are introduced.
3. FFI ownership and memory-release behavior must be explicit and single-path at boundaries.
4. CI changes must increase observability without masking failures.
5. Release decision must be evidence-based from mandatory gate snapshots.

## 4. Day-by-Day Plan

### Day 1
- Plan/Design:
  - Freeze Node ownership closure target and acceptance tests.
  - Define what counts as "explicit ownership guarantee" vs "marshaling-dependent behavior".
- Implement:
  - Harden Node boundary behavior and extend regression loops/payload edges.
- Validate:
  - Node parity + Node regression pass with increased stress patterns.
- Deliverables:
  - Node ownership closure design note
  - strengthened Node regression cases (v2)

### Day 2
- Implement:
  - Integrate `scripts/ci/check-version-consistency.sh` into `ci-gates` workflow.
  - Improve CI summary clarity for version mismatch and gate-failure triage.
- Validate:
  - CI workflow definition includes version gate in deterministic order.
  - mismatch scenario is detectable with clear summary signal.
- Deliverables:
  - version-consistency gate in CI
  - summary signal update for mismatch/failure hints

### Day 3
- Implement:
  - Create cross-platform validation playbook with canonical commands and fallbacks.
  - Stabilize Python smoke invocation path around `uv` cache behavior.
- Validate:
  - documented commands reproduce parity/gate checks on current Windows baseline.
  - fallback guidance is explicit for bash-dependent paths.
- Deliverables:
  - cross-platform validation playbook
  - stable local Python smoke invocation guidance

### Day 4
- Implement:
  - Promote WASM smoke diagnostics from basic pass/fail to actionable triage mapping.
  - Ensure release-readiness checklist references WASM signal expectations consistently.
- Validate:
  - WASM failure output points to actionable toolchain/runtime notes.
  - checklist and workflow signal language remain aligned.
- Deliverables:
  - hardened WASM triage guidance
  - checklist/workflow wording alignment

### Day 5
- Implement:
  - Expand conformance depth with targeted invalid boundary and mixed stress additions.
  - Keep deterministic location/code expectations explicit.
- Validate:
  - conformance runner remains green after expansion.
  - new cases remain schema-valid and index-consistent.
- Deliverables:
  - new conformance vectors (targeted)
  - updated `tests/conformance/INDEX.md`

### Day 6
- Implement:
  - Prepare release execution package:
    - release notes draft
    - publish order/runbook
    - go/no-go checklist completion view
- Validate:
  - all mandatory gate commands are rerun and snapshotted.
  - unresolved blockers are listed with concrete next trigger conditions.
- Deliverables:
  - release package draft
  - final blocker/risk ledger

### Day 7
- Validate/Document/Decision:
  - Apply go/no-go criteria.
  - If Go: execute `v1.0.0` release flow and record evidence.
  - If No-Go: publish release-prep closeout with explicit blocker set and next release trigger.
- Deliverables:
  - `docs/week5-review-summary.md`
  - either release execution record or release-prep closeout note

## 5. Definition of Done (Week 5)
1. Node ownership risk is reduced to explicitly validated contract behavior.
2. Version consistency check is part of routine CI gate execution.
3. Cross-platform validation playbook exists and is reproducible.
4. WASM smoke is an actionable release-readiness signal.
5. Conformance diagnostic depth increases again without regression.
6. Week 5 ends with either:
- executed `v1.0.0` release, or
- release-prep closeout with explicit blockers and next trigger conditions.

## 6. Risks and Mitigations
1. Risk: Node ownership closure may require boundary changes with subtle regression potential.
- Mitigation: add stress-focused regression first, then harden internals under fixed parity expectations.

2. Risk: CI gate expansion can introduce ordering/noise issues.
- Mitigation: keep one deterministic gate order and preserve stable summary schema.

3. Risk: Windows-local validation still diverges from bash-dependent Linux CI paths.
- Mitigation: formalize fallback commands and explicitly mark non-executable local paths.

4. Risk: Release schedule pressure may force premature go decision.
- Mitigation: enforce objective go/no-go checklist and require blocker-free mandatory gate snapshot.

## 7. Priority Order
1. Node ownership contract closure
2. CI version-gate integration
3. Cross-platform validation reliability
4. WASM triage signal hardening
5. Conformance depth expansion
6. Release decision package and execution/closeout

## 8. Validation Commands (Target)
- `cargo test --offline -p ccml-core`
- `cargo test --offline -p ccml-ffi`
- `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
- `UV_CACHE_DIR=.uv-cache uv run python tests/smoke.py`
- `powershell -ExecutionPolicy Bypass -File tests/bindings/run-parity-smoke.ps1`
- `node bindings/node/tests/day1-regression.mjs`
- `bash scripts/ci/check-version-consistency.sh`
- `bash scripts/ci/run-ci-gates.sh`
- `bash scripts/wasm/build-and-run-smoke.sh`

## 9. Execution Log
### Day 1
- Plan:
  - Fixed Day 1 scope to Node ownership closure only; no parser/core semantic changes.
  - Locked acceptance criteria to explicit allocation/decode/free behavior at Node boundary.
- Design:
  - Replaced marshaling-dependent out-string flow with pointer-ownership flow aligned to `ccml_free`.
  - Added deterministic cleanup path (`finally`) so partial failures still release any outstanding pointer.
- Implement:
  - Updated Node FFI wrapper to consume owned output pointers and free them explicitly:
    - `bindings/node/src/ffi.js`
  - Expanded Node Day 1 regression coverage:
    - increased mixed-call loop count
    - added large payload success checks
    - added multiple invalid-input error checks
    - `bindings/node/tests/day1-regression.mjs`
- Validate:
  - `node bindings/node/tests/day2-smoke.mjs`
  - `node bindings/node/tests/day1-regression.mjs`
- Document:
  - Added Day 1 ownership design note:
    - `docs/node-memory-ownership-day1.md`

### Day 2
- Plan:
  - Integrate version consistency check into the existing CI gate flow without changing gate semantics.
  - Keep summary artifact format stable while adding explicit version mismatch visibility.
- Design:
  - Add a dedicated `version_consistency` gate in `scripts/ci/run-ci-gates.sh` with its own timeout/log/reason fields.
  - Parse `rust/python/node` versions from the gate log and emit them in `artifacts/ci/gate-summary.md`.
  - Extend workflow trigger ownership paths to include `bindings/node/**` so Node version changes are gated.
- Implement:
  - Updated CI gate runner:
    - `scripts/ci/run-ci-gates.sh`
      - added `GATE_TIMEOUT_VERSION_SECONDS`
      - added `version_consistency` gate execution
      - added `version_rust/version_python/version_node` summary fields
      - added `version_consistency_log` and `version_consistency_reason`
      - added failed-gate tail section for version consistency
  - Updated workflow trigger paths:
    - `.github/workflows/ci-gates.yml`
      - added `bindings/node/**` to `push.paths` and `pull_request.paths`
- Validate:
  - File-level checks passed:
    - `rg -n "version_consistency|version_rust|version_python|version_node|GATE_TIMEOUT_VERSION_SECONDS|version_consistency_log" scripts/ci/run-ci-gates.sh`
    - `rg -n "bindings/node/\\*\\*" .github/workflows/ci-gates.yml`
  - Runtime validation status:
    - `bash scripts/ci/check-version-consistency.sh` could not run locally because `bash` is unavailable in this Windows environment.
- Document:
  - Recorded Day 2 implementation and validation status in this execution log.

### Day 3
- Plan:
  - Produce one cross-platform validation playbook with explicit Windows and bash-platform command sets.
  - Stabilize local Python smoke execution around repo-local `uv` cache usage.
- Design:
  - Keep CI bash path unchanged, and add a Windows PowerShell fallback only for local reproducibility.
  - Define fallback rules for bash absence, uv cache collisions, and dynamic library load failures.
- Implement:
  - Added cross-platform validation playbook:
    - `docs/cross-platform-validation-playbook.md`
  - Added Windows version-consistency fallback script:
    - `scripts/ci/check-version-consistency.ps1`
- Validate:
  - `powershell -ExecutionPolicy Bypass -File scripts/ci/check-version-consistency.ps1` -> passed (`0.1.0` aligned)
  - `powershell -ExecutionPolicy Bypass -File tests/bindings/run-parity-smoke.ps1` -> passed
  - `Set-Location bindings/python; $env:UV_CACHE_DIR='.uv-cache'; uv run python tests/smoke.py` -> passed
- Document:
  - Recorded platform-specific commands, fallbacks, and minimum evidence requirements in playbook.

### Day 4
- Plan:
- Design:
- Implement:
- Validate:
- Document:

### Day 5
- Plan:
- Design:
- Implement:
- Validate:
- Document:

### Day 6
- Plan:
- Design:
- Implement:
- Validate:
- Document:

### Day 7
- Plan:
- Design:
- Implement:
- Validate:
- Document:
