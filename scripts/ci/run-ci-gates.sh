#!/usr/bin/env bash
set -u

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"

mkdir -p artifacts/ci
SUMMARY_PATH="artifacts/ci/gate-summary.md"
FFI_LIB_PATH="$REPO_ROOT/core/rust/target/debug/libccml_ffi.so"

status_core="pass"
status_ffi="pass"
status_ffi_build="pass"
status_conformance="pass"
status_python="pass"

run_gate() {
  local gate_name="$1"
  shift
  if "$@"; then
    return 0
  fi
  echo "gate failed: ${gate_name}" >&2
  return 1
}

if ! run_gate "core" bash -lc "cd core/rust && cargo test --offline -p ccml-core"; then
  status_core="fail"
fi

if ! run_gate "ffi" bash -lc "cd core/rust && cargo test --offline -p ccml-ffi"; then
  status_ffi="fail"
fi

if ! run_gate "ffi_build" bash -lc "cd core/rust && cargo build --offline -p ccml-ffi"; then
  status_ffi_build="fail"
fi

if ! run_gate "conformance" bash -lc "cd core/rust && cargo run --offline -p ccml-cli -- conformance ../../tests/conformance"; then
  status_conformance="fail"
fi

if ! run_gate "python_smoke" bash -lc "cd '$REPO_ROOT/bindings/python' && UV_CACHE_DIR=.uv-cache CCML_FFI_LIB='$FFI_LIB_PATH' uv run python tests/smoke.py"; then
  status_python="fail"
fi

overall="pass"
if [[ "$status_core" != "pass" || "$status_ffi" != "pass" || "$status_ffi_build" != "pass" || "$status_conformance" != "pass" || "$status_python" != "pass" ]]; then
  overall="fail"
fi

cat >"$SUMMARY_PATH" <<EOF
# CI Gate Summary

- overall: ${overall}
- core: ${status_core}
- ffi: ${status_ffi}
- ffi_build: ${status_ffi_build}
- conformance: ${status_conformance}
- python_smoke: ${status_python}
EOF

if [[ "$overall" != "pass" ]]; then
  exit 1
fi
