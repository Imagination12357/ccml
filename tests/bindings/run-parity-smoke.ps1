$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\\..")

Write-Output "== Node parity smoke =="
node (Join-Path $repoRoot "bindings\\node\\tests\\day2-smoke.mjs")

Write-Output "== Python parity smoke =="
Push-Location (Join-Path $repoRoot "bindings\\python")
try {
    $env:UV_CACHE_DIR = ".uv-cache"
    uv run python tests\\smoke.py
}
finally {
    Pop-Location
}

Write-Output "parity smoke: ok"
