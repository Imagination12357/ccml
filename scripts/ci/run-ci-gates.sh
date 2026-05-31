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

GATE_TIMEOUT_CORE_SECONDS="${GATE_TIMEOUT_CORE_SECONDS:-900}"
GATE_TIMEOUT_FFI_SECONDS="${GATE_TIMEOUT_FFI_SECONDS:-900}"
GATE_TIMEOUT_FFI_BUILD_SECONDS="${GATE_TIMEOUT_FFI_BUILD_SECONDS:-600}"
GATE_TIMEOUT_CONFORMANCE_SECONDS="${GATE_TIMEOUT_CONFORMANCE_SECONDS:-900}"
GATE_TIMEOUT_PYTHON_SECONDS="${GATE_TIMEOUT_PYTHON_SECONDS:-600}"
GATE_TIMEOUT_VERSION_SECONDS="${GATE_TIMEOUT_VERSION_SECONDS:-180}"

status_core="pass"
status_ffi="pass"
status_ffi_build="pass"
status_conformance="pass"
status_python="pass"
status_version="pass"
reason_core="ok"
reason_ffi="ok"
reason_ffi_build="ok"
reason_conformance="ok"
reason_python="ok"
reason_version="ok"

core_passed=0
core_failed=0
ffi_passed=0
ffi_failed=0
conformance_total=0
conformance_failed=0
python_passed=0
python_failed=0
version_rust="n/a"
version_python="n/a"
version_node="n/a"

run_gate() {
  local gate_name="$1"
  local log_path="$2"
  local timeout_seconds="$3"
  shift
  shift
  shift
  if timeout --preserve-status "${timeout_seconds}" "$@" >"$log_path" 2>&1; then
    echo "ok"
    return 0
  fi
  local rc=$?
  if [[ "$rc" -eq 124 ]]; then
    echo "timeout(${timeout_seconds}s)"
  else
    echo "exit(${rc})"
  fi
  echo "gate failed: ${gate_name}" >&2
  return "$rc"
}

tail_snippet() {
  local log_path="$1"
  if [[ ! -f "$log_path" ]]; then
    echo "(log file missing)"
    return 0
  fi
  tail -n 20 "$log_path" | sed 's/^/    /'
  return 0
}

reason_core="$(run_gate "core" "$LOG_DIR/core.log" "$GATE_TIMEOUT_CORE_SECONDS" bash -lc "cd core/rust && cargo test --offline -p ccml-core")"
if [[ "$reason_core" != "ok" ]]; then
  status_core="fail"
fi
core_passed="$(grep -Eo '[0-9]+ passed' "$LOG_DIR/core.log" | awk '{sum += $1} END {print sum+0}')"
core_failed="$(grep -Eo '[0-9]+ failed' "$LOG_DIR/core.log" | awk '{sum += $1} END {print sum+0}')"

reason_ffi="$(run_gate "ffi" "$LOG_DIR/ffi.log" "$GATE_TIMEOUT_FFI_SECONDS" bash -lc "cd core/rust && cargo test --offline -p ccml-ffi")"
if [[ "$reason_ffi" != "ok" ]]; then
  status_ffi="fail"
fi
ffi_passed="$(grep -Eo '[0-9]+ passed' "$LOG_DIR/ffi.log" | awk '{sum += $1} END {print sum+0}')"
ffi_failed="$(grep -Eo '[0-9]+ failed' "$LOG_DIR/ffi.log" | awk '{sum += $1} END {print sum+0}')"

reason_ffi_build="$(run_gate "ffi_build" "$LOG_DIR/ffi_build.log" "$GATE_TIMEOUT_FFI_BUILD_SECONDS" bash -lc "cd core/rust && cargo build --offline -p ccml-ffi")"
if [[ "$reason_ffi_build" != "ok" ]]; then
  status_ffi_build="fail"
fi

reason_conformance="$(run_gate "conformance" "$LOG_DIR/conformance.log" "$GATE_TIMEOUT_CONFORMANCE_SECONDS" bash -lc "cd core/rust && cargo run --offline -p ccml-cli -- conformance ../../tests/conformance")"
if [[ "$reason_conformance" != "ok" ]]; then
  status_conformance="fail"
fi
conformance_line="$(grep -E 'Conformance summary: total=[0-9]+, failed=[0-9]+' "$LOG_DIR/conformance.log" | tail -n 1)"
if [[ -n "$conformance_line" ]]; then
  conformance_total="$(echo "$conformance_line" | sed -E 's/.*total=([0-9]+), failed=([0-9]+).*/\1/')"
  conformance_failed="$(echo "$conformance_line" | sed -E 's/.*total=([0-9]+), failed=([0-9]+).*/\2/')"
fi

reason_python="$(run_gate "python_smoke" "$LOG_DIR/python_smoke.log" "$GATE_TIMEOUT_PYTHON_SECONDS" bash -lc "cd '$REPO_ROOT/bindings/python' && UV_CACHE_DIR=.uv-cache CCML_FFI_LIB='$FFI_LIB_PATH' uv run python tests/smoke.py")"
if [[ "$reason_python" != "ok" ]]; then
  status_python="fail"
fi
if [[ "$status_python" == "pass" ]]; then
  python_passed=1
  python_failed=0
else
  python_passed=0
  python_failed=1
fi

reason_version="$(run_gate "version_consistency" "$LOG_DIR/version_consistency.log" "$GATE_TIMEOUT_VERSION_SECONDS" bash -lc "cd '$REPO_ROOT' && bash scripts/ci/check-version-consistency.sh")"
if [[ "$reason_version" != "ok" ]]; then
  status_version="fail"
fi
version_rust="$(grep -E '^rust_version=' "$LOG_DIR/version_consistency.log" | tail -n 1 | cut -d= -f2- || true)"
version_python="$(grep -E '^python_version=' "$LOG_DIR/version_consistency.log" | tail -n 1 | cut -d= -f2- || true)"
version_node="$(grep -E '^node_version=' "$LOG_DIR/version_consistency.log" | tail -n 1 | cut -d= -f2- || true)"
version_rust="${version_rust:-<missing>}"
version_python="${version_python:-<missing>}"
version_node="${version_node:-<missing>}"

overall="pass"
if [[ "$status_core" != "pass" || "$status_ffi" != "pass" || "$status_ffi_build" != "pass" || "$status_conformance" != "pass" || "$status_python" != "pass" || "$status_version" != "pass" ]]; then
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
- version_consistency: ${status_version}
- version_rust: ${version_rust}
- version_python: ${version_python}
- version_node: ${version_node}
- log_dir: ${LOG_DIR}
- core_log: ${LOG_DIR}/core.log
- ffi_log: ${LOG_DIR}/ffi.log
- ffi_build_log: ${LOG_DIR}/ffi_build.log
- conformance_log: ${LOG_DIR}/conformance.log
- python_smoke_log: ${LOG_DIR}/python_smoke.log
- version_consistency_log: ${LOG_DIR}/version_consistency.log
- core_reason: ${reason_core}
- ffi_reason: ${reason_ffi}
- ffi_build_reason: ${reason_ffi_build}
- conformance_reason: ${reason_conformance}
- python_smoke_reason: ${reason_python}
- version_consistency_reason: ${reason_version}
EOF

{
  echo
  echo "## Failed Gates (Tail Snippets)"
  if [[ "$status_core" == "fail" ]]; then
    echo
    echo "### core (${reason_core})"
    tail_snippet "$LOG_DIR/core.log"
  fi
  if [[ "$status_ffi" == "fail" ]]; then
    echo
    echo "### ffi (${reason_ffi})"
    tail_snippet "$LOG_DIR/ffi.log"
  fi
  if [[ "$status_ffi_build" == "fail" ]]; then
    echo
    echo "### ffi_build (${reason_ffi_build})"
    tail_snippet "$LOG_DIR/ffi_build.log"
  fi
  if [[ "$status_conformance" == "fail" ]]; then
    echo
    echo "### conformance (${reason_conformance})"
    tail_snippet "$LOG_DIR/conformance.log"
  fi
  if [[ "$status_python" == "fail" ]]; then
    echo
    echo "### python_smoke (${reason_python})"
    tail_snippet "$LOG_DIR/python_smoke.log"
  fi
  if [[ "$status_version" == "fail" ]]; then
    echo
    echo "### version_consistency (${reason_version})"
    tail_snippet "$LOG_DIR/version_consistency.log"
  fi
} >>"$SUMMARY_PATH"

if [[ "$overall" != "pass" ]]; then
  exit 1
fi
