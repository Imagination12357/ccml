#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
RUST_WORKSPACE_ROOT="$REPO_ROOT/core/rust"
SUMMARY_PATH="$REPO_ROOT/artifacts/ci/wasm-smoke-summary.md"
CURRENT_STEP="init"

OUT_DIR="$REPO_ROOT/core/rust/target/wasm-smoke"
mkdir -p "$OUT_DIR"
mkdir -p "$(dirname "$SUMMARY_PATH")"

write_summary() {
  local status="$1"
  local detail="$2"
  local current_wasm_bindgen="${CURRENT_WASM_BINDGEN_VERSION:-<not checked>}"
  local expected_wasm_bindgen="${WASM_BINDGEN_VERSION:-<not detected>}"

  cat >"$SUMMARY_PATH" <<EOF
# WASM Runtime Smoke Summary

- status: ${status}
- step: ${CURRENT_STEP}
- detail: ${detail}
- rustc: $(rustc --version 2>/dev/null || echo "<missing>")
- cargo: $(cargo --version 2>/dev/null || echo "<missing>")
- node: $(node --version 2>/dev/null || echo "<missing>")
- expected_wasm_bindgen: ${expected_wasm_bindgen}
- current_wasm_bindgen: ${current_wasm_bindgen}
- out_dir: ${OUT_DIR}
- wasm_entry: ${OUT_DIR}/ccml_wasm.js

## Triage
1. If target setup failed, run `rustup target add wasm32-unknown-unknown`.
2. If package build failed, inspect `ccml-wasm` compile errors under `core/rust`.
3. If bindgen failed, compare `expected_wasm_bindgen` with `current_wasm_bindgen`.
4. If runtime smoke failed, inspect `tests/wasm/runtime-smoke.mjs` and generated entry path.
EOF
}

on_error() {
  local rc="$?"
  write_summary "fail" "exit(${rc})"
  exit "$rc"
}

trap on_error ERR

CURRENT_STEP="rustup target add wasm32-unknown-unknown"
echo "== wasm smoke: ${CURRENT_STEP} =="
rustup target add wasm32-unknown-unknown

CURRENT_STEP="detect wasm-bindgen version"
echo "== wasm smoke: ${CURRENT_STEP} =="
WASM_BINDGEN_VERSION="$(
  awk '
    $0 ~ /^name = "wasm-bindgen"$/ { in_pkg=1; next }
    in_pkg && $0 ~ /^version = / {
      gsub(/version = "/, "", $0)
      gsub(/"/, "", $0)
      print $0
      exit
    }
  ' "$RUST_WORKSPACE_ROOT/Cargo.lock"
)"

if [[ -z "$WASM_BINDGEN_VERSION" ]]; then
  echo "failed to detect wasm-bindgen version from Cargo.lock" >&2
  exit 1
fi

CURRENT_WASM_BINDGEN_VERSION=""
if command -v wasm-bindgen >/dev/null 2>&1; then
  CURRENT_STEP="detect installed wasm-bindgen version"
  echo "== wasm smoke: ${CURRENT_STEP} =="
  CURRENT_WASM_BINDGEN_VERSION="$(wasm-bindgen --version | awk '{print $2}')"
fi

if [[ "$CURRENT_WASM_BINDGEN_VERSION" != "$WASM_BINDGEN_VERSION" ]]; then
  CURRENT_STEP="install wasm-bindgen-cli ${WASM_BINDGEN_VERSION}"
  echo "== wasm smoke: ${CURRENT_STEP} =="
  cargo install --locked wasm-bindgen-cli --version "$WASM_BINDGEN_VERSION" --force
  CURRENT_WASM_BINDGEN_VERSION="$WASM_BINDGEN_VERSION"
fi

CURRENT_STEP="cargo build ccml-wasm"
echo "== wasm smoke: ${CURRENT_STEP} =="
(
  cd "$RUST_WORKSPACE_ROOT"
  cargo build --release -p ccml-wasm --target wasm32-unknown-unknown
)

CURRENT_STEP="wasm-bindgen node packaging"
echo "== wasm smoke: ${CURRENT_STEP} =="
wasm-bindgen \
  --target nodejs \
  --out-dir "$OUT_DIR" \
  "$REPO_ROOT/core/rust/target/wasm32-unknown-unknown/release/ccml_wasm.wasm"

CURRENT_STEP="node runtime smoke"
echo "== wasm smoke: ${CURRENT_STEP} =="
WASM_JS_ENTRY="$OUT_DIR/ccml_wasm.js" node "$REPO_ROOT/tests/wasm/runtime-smoke.mjs"
write_summary "pass" "ok"
