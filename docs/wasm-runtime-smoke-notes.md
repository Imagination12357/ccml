# WASM Runtime Smoke Notes

## Purpose
Validate `ccml-wasm` exports in a real JS runtime path (Node) after `wasm-bindgen` packaging.

## Scope
The smoke verifies:
1. success transcode path (`ccml_to_json_wasm`)
2. parse error propagation (`ccml parse error`)
3. warning visibility via diagnose output (`CCML2001`)

## Entrypoints
1. Build and run script:
- `scripts/wasm/build-and-run-smoke.sh`
2. Runtime smoke test:
- `tests/wasm/runtime-smoke.mjs`
3. CI workflow:
- `.github/workflows/wasm-runtime-smoke.yml`

## Command
```bash
bash scripts/wasm/build-and-run-smoke.sh
```

## Notes
1. Script installs `wasm-bindgen-cli` automatically when missing.
2. Generated JS/WASM artifacts are placed in:
- `core/rust/target/wasm-smoke`
