$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$repoRoot = Resolve-Path (Join-Path $scriptDir "..\\..")

$rustManifest = Join-Path $repoRoot "core\\rust\\Cargo.toml"
$pythonManifest = Join-Path $repoRoot "bindings\\python\\pyproject.toml"
$nodeManifest = Join-Path $repoRoot "bindings\\node\\package.json"

function Extract-Version {
    param(
        [string]$Path,
        [string]$SectionHeader,
        [string]$Pattern
    )

    $inSection = $false
    foreach ($line in Get-Content $Path) {
        if ($line -match "^\[") {
            if ($line -eq $SectionHeader) {
                $inSection = $true
                continue
            }
            if ($inSection) {
                break
            }
        }
        if ($inSection -and $line -match $Pattern) {
            return $Matches[1]
        }
    }
    return $null
}

$rustVersion = Extract-Version -Path $rustManifest -SectionHeader "[workspace.package]" -Pattern '^\s*version\s*=\s*"([^"]+)"\s*$'
$pythonVersion = Extract-Version -Path $pythonManifest -SectionHeader "[project]" -Pattern '^\s*version\s*=\s*"([^"]+)"\s*$'
$nodeJson = Get-Content $nodeManifest -Raw | ConvertFrom-Json
$nodeVersion = $nodeJson.version

if ([string]::IsNullOrWhiteSpace($rustVersion) -or [string]::IsNullOrWhiteSpace($pythonVersion) -or [string]::IsNullOrWhiteSpace($nodeVersion)) {
    Write-Error "version consistency check failed: missing version field(s)"
}

Write-Output "rust_version=$rustVersion"
Write-Output "python_version=$pythonVersion"
Write-Output "node_version=$nodeVersion"

if ($rustVersion -ne $pythonVersion -or $rustVersion -ne $nodeVersion) {
    Write-Error "version consistency check failed: mismatch detected"
}

Write-Output "version consistency check: ok"
