# Node Memory Contract Hardening Targets (Week 4 Day 1)

## Scope
Define regression targets that are independent from parser semantics and focused on Node FFI boundary stability.

## Hardening Targets
1. Repeated success calls keep stable output and do not corrupt subsequent calls.
2. Error path (`status=3`) after success does not poison the next success call.
3. Warn-path diagnostics retrieval remains stable across mixed success/error sequences.
4. Wrapper throws typed `CcmlFfiError` consistently on parse-error path.

## Regression Matrix v1
1. success -> error -> warn mixed cycle, repeated 150 times.
2. Assertions per cycle:
- success output exact match with shared parity fixture
- error status exact match with shared parity fixture
- warn code/severity exact match with shared parity fixture
3. Shared fixture source:
- `tests/bindings/parity-smoke-cases.json`

## Test Entrypoint
- `node bindings/node/tests/day1-regression.mjs`
- package script: `npm run smoke:day1:regression` (under `bindings/node`)

## Notes
- This check is a runtime-stability regression, not a memory-leak proof.
- Explicit caller-managed pointer lifecycle hardening remains a follow-up item.
