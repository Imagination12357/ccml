# Weekly Plan 3 (2026-05-12 to 2026-05-18)

## 1. Weekly Goal
1. Complete Node binding phase-1 skeleton with Python parity smoke coverage.
2. Establish CI gating baseline for core/ffi/conformance/python smoke paths.
3. Add WASM runtime smoke verification in JS runtime.
4. Expand conformance vectors from 46 to >= 60 without changing core parser semantics.
5. Close Week 2 deferred gaps required for Week 4 QA hardening.

## 2. Scope
- In scope:
  - `bindings/node` skeleton and minimal adapter (`to_json`, `diagnose`, memory free contract)
  - Node smoke tests for success/error/warn parity with Python thin adapter behavior
  - CI jobs for:
    - `cargo test --offline -p ccml-core`
    - `cargo test --offline -p ccml-ffi`
    - `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
    - `uv run python tests/smoke.py`
  - WASM runtime smoke harness for exported functions
  - Conformance vector expansion to >= 60 with focused edge classes
- Out of scope:
  - full Node package release flow (`npm publish`)
  - full release automation and signing
  - spec feature expansion beyond current CCML 1.0 draft behavior

## 3. Non-Negotiable Constraints for Week 3
1. `ccml-core` remains the single semantic source of truth.
2. No binding-specific parsing logic is allowed in Python/Node/WASM layers.
3. FFI heap ownership stays single-path: memory returned by FFI must be released via `ccml_free`.
4. Conformance expansion must not weaken existing diagnostics contracts (`code/line/column`).
5. JSON output compatibility guarantees remain strict.

## 4. Day-by-Day Plan

### Day 1 - Tue 2026-05-12
- Plan/Design:
  - Freeze Node phase-1 adapter contract and library loading policy.
  - Define parity checklist against Python smoke behavior.
- Implement:
  - Create `bindings/node` project skeleton.
  - Wire dynamic library loading and base call path.
- Validate:
  - Basic module load and FFI symbol resolution.
- Deliverables:
  - Node binding skeleton
  - parity checklist draft

### Day 2 - Wed 2026-05-13
- Implement:
  - Add Node `to_json` and `diagnose` minimal wrappers.
  - Implement error payload mapping and memory free path.
- Validate:
  - Node happy-path smoke for transcode.
  - Node parse-error smoke for status/payload contract.
- Deliverables:
  - Node phase-1 adapter baseline
  - initial Node smoke tests

### Day 3 - Thu 2026-05-14
- Implement:
  - Add Node warn-path smoke (`CCML2001`) and parity assertions.
  - Normalize cross-binding smoke assertions (Python vs Node).
- Validate:
  - success/error/warn parity across Python and Node.
- Deliverables:
  - Node parity smoke coverage equivalent to Python thin adapter

### Day 4 - Fri 2026-05-15
- Implement:
  - Add CI workflow baseline jobs for core/ffi/conformance/python smoke.
  - Add pass/fail summary artifact output.
- Validate:
  - CI run with expected pass status on current branch.
- Deliverables:
  - CI gating foundation workflow
  - first CI summary artifact

### Day 5 - Sat 2026-05-16
- Implement:
  - Add WASM runtime smoke harness in JS runtime.
  - Verify exported function behavior (`to_json`, parse error payload, diagnose warnings).
- Validate:
  - runtime smoke passes end-to-end.
- Deliverables:
  - WASM runtime smoke script/tests
  - verification notes

### Day 6 - Sun 2026-05-17
- Implement:
  - Expand conformance vectors from 46 to >= 60.
  - Focus classes:
    - control/unicode escape boundaries
    - mixed delimiters/layout corner cases
    - diagnostic boundary positions
- Validate:
  - `ccml-cli conformance` remains green with expanded vectors.
- Deliverables:
  - new conformance cases + updated index summary

### Day 7 - Mon 2026-05-18
- Validate/Document:
  - Re-run weekly quality gates.
  - Capture pass-rate snapshot, remaining risks, and Week 4 handoff priorities.
- Deliverables:
  - `docs/week3-review-summary.md`
  - `docs/sprint4-backlog-draft.md`

## 5. Definition of Done (Week 3)
1. Node phase-1 adapter (`to_json`, `diagnose`, memory free) is implemented and smoke-tested.
2. Node smoke covers success/error/warn and matches Python thin adapter expectations.
3. CI runs core/ffi/conformance/python smoke checks on each change path.
4. WASM runtime smoke passes in JS runtime for baseline exports.
5. Conformance vectors are expanded to >= 60 and conformance run remains passing.
6. Week 3 review and Sprint 4 backlog draft are documented.

## 6. Risks and Mitigations
1. Risk: cross-runtime contract drift (Python vs Node vs WASM).
- Mitigation: keep one shared parity checklist and fail smoke on mismatch.

2. Risk: CI instability due to environment/toolchain differences.
- Mitigation: pin commands and keep workflow minimal before adding matrix complexity.

3. Risk: conformance expansion introduces ambiguous expected diagnostics.
- Mitigation: add vectors incrementally and keep diagnostics assertions explicit.

4. Risk: accidental semantic drift while adding bindings/tests.
- Mitigation: keep all behavioral assertions anchored on `ccml-core` outputs.

## 7. Priority Order
1. Node binding parity baseline
2. CI gating foundation
3. WASM runtime smoke
4. Conformance expansion to >= 60
5. Week 3 review + Sprint 4 handoff docs

## 8. Validation Commands (Target)
- `cargo test --offline -p ccml-core`
- `cargo test --offline -p ccml-ffi`
- `cargo run --offline -p ccml-cli -- conformance ../../tests/conformance`
- `uv run python tests/smoke.py`
- Node smoke command (to be finalized with `bindings/node` scaffold)
- WASM runtime smoke command (to be finalized with harness setup)

## 9. Execution Log
### Day 1 - Tue 2026-05-12
- Plan:
  - Define Node phase-1 boundary and parity targets against Python thin adapter.
  - Build Node skeleton without changing `ccml-core` or FFI semantics.
- Design:
  - `bindings/node` owns runtime integration only.
  - Env override + fallback library resolution policy mirrors Python adapter.
  - Day 1 keeps symbol list and load contract explicit; functional wrappers deferred to Day 2.
- Implement:
  - Added `bindings/node` skeleton:
    - `package.json`
    - `src/contract.js`
    - `src/loader.js`
    - `src/index.js`
    - `tests/day1-smoke.mjs`
  - Added Node parity checklist draft:
    - `docs/node-phase1-parity-checklist.md`
- Validate:
  - `node bindings/node/tests/day1-smoke.mjs` passed.
  - Verified FFI export symbols from `core/rust/target/debug/ccml_ffi.dll`:
    - `ccml_to_json`
    - `ccml_diagnose`
    - `ccml_version`
    - `ccml_free`
- Document:
  - Recorded Day 1 implementation and validation status in this log.

### Day 2 - Wed 2026-05-13
- Plan:
  - Implement Node minimal wrappers over `ccml-ffi` for `to_json` and `diagnose`.
  - Add error payload/status mapping aligned with Python adapter behavior.
- Design:
  - Use `koffi` as Node-side FFI layer.
  - Keep wrapper surface minimal (`createCcmlFfi`, `CcmlFfiError`) and no parser logic in Node.
- Implement:
  - Updated `bindings/node/package.json`:
    - added `koffi` dependency
    - added `smoke:day2` script
  - Added `bindings/node/src/ffi.js`:
    - `createCcmlFfi()` with `toJson()` and `diagnose()` wrappers
    - status-based error mapping to `CcmlFfiError`
    - JSON error payload decode path
  - Added `bindings/node/tests/day2-smoke.mjs`:
    - success path assertion
    - parse error status assertion (`3`)
    - warning assertion (`CCML2001`)
- Validate:
  - `node bindings/node/tests/day2-smoke.mjs` passed.
  - During implementation, invalid `koffi` out-parameter signatures caused FFI panics; fixed by aligning with `char**` out contract.
- Document:
  - Day 2 status recorded here.
  - Note: direct caller-managed pointer release (`ccml_free`) integration in Node wrapper is still a follow-up hardening item due current `koffi` string out-marshal behavior.
