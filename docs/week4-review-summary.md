# Week 4 Review Summary (2026-05-19 to 2026-05-25)

## 1) Outcome Snapshot
- Node runtime boundary regression baseline added for repeated mixed call cycles.
- CI gate summary now includes per-gate reasons, log pointers, and failed-gate tail snippets.
- CI gate workflow trigger scope/concurrency policy tightened for active ownership paths.
- Release-readiness baseline checklist and version consistency checker were added.
- Conformance quality depth expanded from 60 to 64 cases with targeted invalid/warn additions.

## 2) Validation Results (Day 7 Snapshot)
- `ccml-core` tests: 10 passed, 0 failed.
- `ccml-ffi` tests: 9 passed, 0 failed.
- Conformance runner (`ccml-cli`): `total=64`, `failed=0`.
- Node parity smoke (`tests/bindings/run-parity-smoke.ps1`): passed.
- Node Day 1 regression (`bindings/node/tests/day1-regression.mjs`): passed (`150 mixed cycles`).
- Python smoke (`uv run python tests/smoke.py`): direct command failed due local `uv` cache init conflict, but passed via parity script path (`UV_CACHE_DIR=.uv-cache`).
- WASM runtime smoke (`bash scripts/wasm/build-and-run-smoke.sh`): not executed in this Windows environment due missing `bash`.

## 3) Week 4 DoD Check
1. Node memory-contract hardening with regression coverage: met (runtime-sequence regression baseline).
2. CI artifacts provide clear, per-gate actionable failure pointers: met.
3. WASM runtime smoke is visible as maintained signal with triage-friendly diagnostics: partially met (CI path prepared; local Windows execution unavailable).
4. Release-readiness checklist and version consistency baseline checks are in place: met.
5. Conformance quality depth increased without runner regression: met (`64/0`).
6. Week 4 review and Sprint 5 backlog draft documented: met.

## 3.1) DoD Completion Estimate
- Estimated completion: **92%**
- Rationale:
  - 5/6 DoD items are fully met.
  - 1/6 item (WASM runtime smoke operational completeness) is partially met due local Windows bash/toolchain path limits, while CI-oriented path is prepared.

## 4) Known Gaps / Risks
- Node wrapper still relies on `koffi` out-string marshaling behavior; explicit caller-managed pointer lifecycle hardening is not fully closed.
- Local Windows validation remains weaker on bash-dependent gate paths (WASM build/run, shell checker scripts).
- Python smoke direct invocation can fail on host-level `uv` cache path conflicts; repo-local cache override is still required for stable execution.
- CI trigger scope is narrowed for current gate ownership paths; any future gate expansion requires explicit path list updates.

## 5) Recommended Week 5 Priorities
1. Close Node memory contract hardening from regression baseline to explicit ownership guarantees.
2. Add cross-platform execution fallbacks for bash-dependent checks where feasible.
3. Promote version consistency checker into CI gate workflow.
4. Continue conformance invalid diagnostic-depth expansion with stable location expectations.
5. Start RC-candidate gate definition from release-readiness checklist blockers.
