#!/usr/bin/env bash
set -u

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"

mkdir -p artifacts/ci
SUMMARY_PATH="artifacts/ci/gate-summary.md"
FFI_LIB_PATH="$REPO_ROOT/core/rust/target/debug/libccml_ffi.so"
LOG_DIR="artifacts/ci/logs"
mkdir -p "$LOG_DIR"

status_core="pass"
status_ffi="pass"
status_ffi_build="pass"
status_conformance="pass"
status_python="pass"

core_passed=0
core_failed=0
ffi_passed=0
ffi_failed=0
conformance_total=0
conformance_failed=0
python_passed=0
python_failed=0

run_gate() {
  local gate_name="$1"
  local log_path="$2"
  shift
  shift
  if "$@" >"$log_path" 2>&1; then
    return 0
  fi
  echo "gate failed: ${gate_name}" >&2
  return 1
}

if ! run_gate "core" "$LOG_DIR/core.log" bash -lc "cd core/rust && cargo test --offline -p ccml-core"; then
  status_core="fail"
fi
core_passed="$(grep -Eo '[0-9]+ passed' "$LOG_DIR/core.log" | awk '{sum += $1} END {print sum+0}')"
core_failed="$(grep -Eo '[0-9]+ failed' "$LOG_DIR/core.log" | awk '{sum += $1} END {print sum+0}')"

if ! run_gate "ffi" "$LOG_DIR/ffi.log" bash -lc "cd core/rust && cargo test --offline -p ccml-ffi"; then
  status_ffi="fail"
fi
ffi_passed="$(grep -Eo '[0-9]+ passed' "$LOG_DIR/ffi.log" | awk '{sum += $1} END {print sum+0}')"
ffi_failed="$(grep -Eo '[0-9]+ failed' "$LOG_DIR/ffi.log" | awk '{sum += $1} END {print sum+0}')"

if ! run_gate "ffi_build" "$LOG_DIR/ffi_build.log" bash -lc "cd core/rust && cargo build --offline -p ccml-ffi"; then
  status_ffi_build="fail"
fi

if ! run_gate "conformance" "$LOG_DIR/conformance.log" bash -lc "cd core/rust && cargo run --offline -p ccml-cli -- conformance ../../tests/conformance"; then
  status_conformance="fail"
fi
conformance_line="$(grep -E 'Conformance summary: total=[0-9]+, failed=[0-9]+' "$LOG_DIR/conformance.log" | tail -n 1)"
if [[ -n "$conformance_line" ]]; then
  conformance_total="$(echo "$conformance_line" | sed -E 's/.*total=([0-9]+), failed=([0-9]+).*/\1/')"
  conformance_failed="$(echo "$conformance_line" | sed -E 's/.*total=([0-9]+), failed=([0-9]+).*/\2/')"
fi

if ! run_gate "python_smoke" "$LOG_DIR/python_smoke.log" bash -lc "cd '$REPO_ROOT/bindings/python' && UV_CACHE_DIR=.uv-cache CCML_FFI_LIB='$FFI_LIB_PATH' uv run python tests/smoke.py"; then
  status_python="fail"
fi
if [[ "$status_python" == "pass" ]]; then
  python_passed=1
  python_failed=0
else
  python_passed=0
  python_failed=1
fi

overall="pass"
if [[ "$status_core" != "pass" || "$status_ffi" != "pass" || "$status_ffi_build" != "pass" || "$status_conformance" != "pass" || "$status_python" != "pass" ]]; then
  overall="fail"
fi

cat >"$SUMMARY_PATH" <<EOF
# CI Gate Summary

- overall: ${overall}
- core: ${status_core}
- core_tests_passed: ${core_passed}
- core_tests_failed: ${core_failed}
- ffi: ${status_ffi}
- ffi_tests_passed: ${ffi_passed}
- ffi_tests_failed: ${ffi_failed}
- ffi_build: ${status_ffi_build}
- conformance: ${status_conformance}
- conformance_total: ${conformance_total}
- conformance_failed: ${conformance_failed}
- python_smoke: ${status_python}
- python_smoke_passed: ${python_passed}
- python_smoke_failed: ${python_failed}
EOF

if [[ "$overall" != "pass" ]]; then
  exit 1
fi
