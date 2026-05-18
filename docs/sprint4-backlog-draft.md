# Sprint 4 Backlog Draft

## Priority 1 - Runtime Contract Hardening (Node FFI)
1. Harden Node wrapper memory ownership behavior around FFI out buffers.
2. Add regression tests for repeated calls and mixed success/error sequences.
3. Keep error/status envelope parity with Python adapter.

## Priority 2 - CI Gate Visibility and Reliability
1. Extend CI summary artifact with clearer per-gate failure pointers (log file links/sections).
2. Add timeout policies for long-running jobs and hanging-risk commands.
3. Keep path-scoped triggers and concurrency cancellation to control queue/runtime cost.

## Priority 3 - WASM Validation Hardening
1. Promote wasm runtime smoke from standalone workflow into release-readiness signal set.
2. Add failure triage notes and expected toolchain/version diagnostics in workflow output.
3. Verify stable behavior across parse-success, parse-error, warn paths with shared parity fixture.

## Priority 4 - Release Readiness Foundation
1. Draft minimal release checklist for crates/Python/Node/WASM artifacts.
2. Add version consistency checks across manifests and release notes template.
3. Define blocker gates for RC candidate branch.

## Priority 5 - Conformance Quality Deepening
1. Expand beyond count target with focused edge cases for invalid diagnostics boundaries.
2. Add cross-runtime parity-focused vectors that stress mixed layout + escape + warning combinations.
3. Keep conformance runner green while tightening diagnostic expectation fidelity.

## Sprint 4 Exit Criteria (Draft)
1. Node memory-contract hardening is implemented with regression coverage.
2. CI gate artifacts provide actionable failure detail without manual log digging.
3. WASM runtime smoke is consistently visible as a maintained gate.
4. Release-readiness checklist and baseline gates are in place for Week 5 handoff.
