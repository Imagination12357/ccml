#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
cd "$REPO_ROOT"

RUST_MANIFEST="core/rust/Cargo.toml"
PY_MANIFEST="bindings/python/pyproject.toml"
NODE_MANIFEST="bindings/node/package.json"

extract_rust_version() {
  awk '
    /^\[workspace\.package\]/ { in_wp=1; next }
    /^\[/ && in_wp { exit }
    in_wp && $0 ~ /^version[[:space:]]*=/ {
      gsub(/version[[:space:]]*=[[:space:]]*"/, "", $0)
      gsub(/"/, "", $0)
      print $0
      exit
    }
  ' "$RUST_MANIFEST"
}

extract_python_version() {
  awk '
    /^\[project\]/ { in_proj=1; next }
    /^\[/ && in_proj { exit }
    in_proj && $0 ~ /^version[[:space:]]*=/ {
      gsub(/version[[:space:]]*=[[:space:]]*"/, "", $0)
      gsub(/"/, "", $0)
      print $0
      exit
    }
  ' "$PY_MANIFEST"
}

extract_node_version() {
  grep -E '"version"[[:space:]]*:' "$NODE_MANIFEST" | head -n 1 | sed -E 's/.*"version"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/'
}

rust_version="$(extract_rust_version)"
python_version="$(extract_python_version)"
node_version="$(extract_node_version)"

if [[ -z "$rust_version" || -z "$python_version" || -z "$node_version" ]]; then
  echo "version consistency check failed: missing version field(s)" >&2
  echo "rust=${rust_version:-<missing>}" >&2
  echo "python=${python_version:-<missing>}" >&2
  echo "node=${node_version:-<missing>}" >&2
  exit 1
fi

echo "rust_version=$rust_version"
echo "python_version=$python_version"
echo "node_version=$node_version"

if [[ "$rust_version" != "$python_version" || "$rust_version" != "$node_version" ]]; then
  echo "version consistency check failed: mismatch detected" >&2
  exit 1
fi

echo "version consistency check: ok"
