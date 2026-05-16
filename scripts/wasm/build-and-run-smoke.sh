#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
RUST_WORKSPACE_ROOT="$REPO_ROOT/core/rust"

OUT_DIR="$REPO_ROOT/core/rust/target/wasm-smoke"
mkdir -p "$OUT_DIR"

rustup target add wasm32-unknown-unknown

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
  CURRENT_WASM_BINDGEN_VERSION="$(wasm-bindgen --version | awk '{print $2}')"
fi

if [[ "$CURRENT_WASM_BINDGEN_VERSION" != "$WASM_BINDGEN_VERSION" ]]; then
  cargo install --locked wasm-bindgen-cli --version "$WASM_BINDGEN_VERSION" --force
fi

(
  cd "$RUST_WORKSPACE_ROOT"
  cargo build --release -p ccml-wasm --target wasm32-unknown-unknown
)

wasm-bindgen \
  --target nodejs \
  --out-dir "$OUT_DIR" \
  "$REPO_ROOT/core/rust/target/wasm32-unknown-unknown/release/ccml_wasm.wasm"

WASM_JS_ENTRY="$OUT_DIR/ccml_wasm.js" node "$REPO_ROOT/tests/wasm/runtime-smoke.mjs"
