# Sprint 5 Backlog Draft

## Priority 1 - Node Memory Contract Closure
1. Replace marshaling-dependent behavior with explicit ownership-safe boundary handling where possible.
2. Add targeted regression cases for repeated mixed calls with larger loop counts and edge payload sizes.
3. Keep Python/Node error envelope parity fixed while hardening internals.

## Priority 2 - CI Gate Integration Hardening
1. Integrate `scripts/ci/check-version-consistency.sh` into `ci-gates` workflow.
2. Keep summary artifact schema stable while adding clearer mismatch/failure hints.
3. Re-check path-scoped trigger ownership mapping when new gates are added.

## Priority 3 - Cross-Platform Validation Reliability
1. Add practical Windows fallback guidance for bash-dependent checks.
2. Investigate and stabilize `uv` cache behavior to reduce host-path collisions.
3. Document canonical local commands per platform for parity/gate validation.

## Priority 4 - WASM Signal Promotion
1. Keep wasm runtime smoke visible as release-readiness signal.
2. Improve triage linkage from wasm failures to actionable diagnostics/toolchain notes.
3. Add parity-focused wasm stress checks beyond baseline smoke as environment allows.

## Priority 5 - Conformance Diagnostic Depth
1. Expand invalid diagnostics around newline/comment/EOF boundary classes.
2. Add mixed stress cases combining escapes/layout/warnings while preserving deterministic expectations.
3. Keep conformance runner green with explicit location/code checks.

## Priority 6 - Release Execution / Release-Prep Track
1. Decide Week 5 release mode:
- release execution (`v1.0.0`) if blocker gates are green
- otherwise explicit release-prep closeout with blocker list and RC criteria
2. Add minimal release gate checklist execution path:
- version consistency
- core/ffi/conformance/parity/wasm signals
- packaging metadata sanity checks
3. Prepare release artifacts/docs baseline:
- release notes draft
- publish order/runbook for Rust/Python/Node/WASM surfaces
4. Define go/no-go decision point and owner-visible sign-off checklist.

## Sprint 5 Exit Criteria (Draft)
1. Node memory ownership risk is reduced from baseline concern to explicitly validated behavior.
2. Version consistency check is part of regular CI gating.
3. Cross-platform validation playbook exists and is reproducible.
4. Conformance suite quality depth increases again without regression.
5. Week 5 ends with either:
- executed release, or
- completed release-prep package with explicit blockers and next release trigger conditions.
