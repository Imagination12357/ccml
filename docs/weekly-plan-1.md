# Weekly Plan 1 (2026-04-28 to 2026-05-04)

## 1. Weekly Goal
1. Rust core implementation kickoff with stable module boundaries.
2. Conformance vectors expansion focused on edge cases.
3. Detailed execution structure for next-step polyglot delivery.

## 2. Scope
- In scope:
  - Rust core skeleton and minimal parser behavior
  - Conformance test vectors (`valid`, `invalid`, `warn`) expansion
  - Diagnostics and warning policy alignment (`keep-last + warn`)
  - Plan/design detail updates for Python/Node/WASM integration
- Out of scope:
  - Full Python/Node binding implementation
  - Production release automation completion

## 3. Day-by-Day Plan

### Day 1 - Tue 2026-04-28
- Freeze Rust workspace and crate responsibilities:
  - `ccml-core`, `ccml-cli`, `ccml-ffi`, `ccml-wasm`
- Freeze core API signatures:
  - `parse(text)`, `to_json(text, options)`, `diagnose(text)`
- Deliverables:
  - updated architecture/service boundary notes
  - crate ownership table

### Day 2 - Wed 2026-04-29
- Expand conformance vectors phase 1 (30-50 target):
  - bare key accepted/disallowed patterns
  - comments and separators
  - implicit root object
  - numeric-looking and keyword-like keys
- Deliverables:
  - new vector files
  - vector index document

### Day 3 - Thu 2026-04-30
- Build minimal Rust parser path in `ccml-core`:
  - object, array, string, number, boolean, null
  - implicit root object behavior
- Deliverables:
  - minimal parse/transcode path
  - initial unit test pass

### Day 4 - Fri 2026-05-01
- Implement diagnostics baseline:
  - error code + line + column
  - duplicate key `keep-last + warn`
- Deliverables:
  - diagnostic output contract implementation
  - warning path tests

### Day 5 - Sat 2026-05-02
- Build conformance runner prototype for Rust:
  - consume shared vector schema
  - verify `valid`, `invalid`, `warn`
- Deliverables:
  - executable local runner
  - pass/fail summary output

### Day 6 - Sun 2026-05-03
- Refine polyglot integration plan:
  - FFI surface draft
  - Python/Node boundary contract
  - WASM packaging notes
- Deliverables:
  - updated polyglot implementation plan
  - binding TODO list

### Day 7 - Mon 2026-05-04
- Weekly validation and sprint-close:
  - conformance pass rate snapshot
  - known gaps and blockers
  - next-week priorities
- Deliverables:
  - weekly review summary
  - Sprint 2 backlog draft

## 4. Definition of Done (Week 1)
1. Rust core supports minimal parse + JSON transcode path.
2. Conformance vectors are expanded to at least 50 total cases.
3. Invalid-case diagnostics validate `code`, `line`, `column`.
4. Warn-case diagnostics validate duplicate-key warning behavior.
5. Polyglot boundary notes are concrete enough for binding kickoff.

## 5. Risks and Mitigations
1. Risk: parser implementation drifts from spec detail.
- Mitigation: conformance-first gating before feature merge.
2. Risk: diagnostics contract changes mid-week.
- Mitigation: freeze required fields (`code`, `line`, `column`) first.
3. Risk: vector quality over quantity imbalance.
- Mitigation: prioritize edge-case classes before bulk additions.

## 6. Priority Order
1. Correctness and compatibility with spec
2. Diagnostic clarity
3. Performance tuning (after correctness gate)
