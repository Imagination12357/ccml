#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"
RUST_WORKSPACE_ROOT="$REPO_ROOT/core/rust"

OUT_DIR="$REPO_ROOT/core/rust/target/wasm-smoke"
mkdir -p "$OUT_DIR"

rustup target add wasm32-unknown-unknown

if ! command -v wasm-bindgen >/dev/null 2>&1; then
  cargo install --locked wasm-bindgen-cli --version 0.2.121
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
