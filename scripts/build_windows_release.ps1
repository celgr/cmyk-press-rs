Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$RustDir = Join-Path $RepoRoot "rust"
$PluginName = "CMYK Press"
$BinaryName = "cmyk_press_renderer"
$ProfileDir = Join-Path $RustDir "target\release"
$BundleDir = Join-Path $ProfileDir "$PluginName.aex"
$DllPath = Join-Path $ProfileDir "$BinaryName.dll"

Push-Location $RustDir
try {
    if (-not $env:AESDK_ROOT) {
        $env:AESDK_ROOT = ""
    }
    cargo build --release
}
finally {
    Pop-Location
}

if (-not (Test-Path $DllPath)) {
    throw "Expected release DLL was not produced: $DllPath"
}

Copy-Item -Force $DllPath $BundleDir
Write-Host "Built $BundleDir"
