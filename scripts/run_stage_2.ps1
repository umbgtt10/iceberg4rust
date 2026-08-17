# Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
# Licensed under the MIT License
# SPDX-License-Identifier: MIT

$ErrorActionPreference = "Stop"
Push-Location (Split-Path $PSScriptRoot -Parent)

function Invoke-Crap4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages,
        [double]$Threshold = 15
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    if (-not (Get-Command cargo-crap4rust -ErrorAction SilentlyContinue)) {
        Write-Host "`ncargo-crap4rust is not installed." -ForegroundColor Red
        Write-Host "Install it with: cargo install cargo-crap4rust" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\Cargo.toml")).Path
    $args = @("--manifest-path", $manifestPath)
    foreach ($package in $Packages) {
        $args += @("--package", $package)
    }
    $args += @("--warn-only", "--threshold", $Threshold.ToString())

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $output = & cargo crap4rust @args 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE
    $output | ForEach-Object { Write-Host $_ }

    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (exit code $exitCode)" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $summaryLine = $output | Select-String -Pattern "summary:\s+total_functions=.*crappy_functions=(\d+)"
    if (-not $summaryLine) {
        Write-Host "`nFailed: $Label (could not parse crap4rust summary)" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $crappyCount = [int]$summaryLine.Matches[0].Groups[1].Value
    if ($crappyCount -gt 0) {
        Write-Host "`nFailed: $Label ($crappyCount crappy functions detected)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}

function Invoke-Twin4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    if (-not (Get-Command cargo-twin4rust -ErrorAction SilentlyContinue)) {
        Write-Host "`ncargo-twin4rust is not installed." -ForegroundColor Red
        Write-Host "Install it with: cargo install cargo-twin4rust" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\Cargo.toml")).Path

    $args = @("twin4rust", "--manifest-path", $manifestPath)
    foreach ($package in $Packages) {
        $args += @("--package", $package)
    }

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $output = & cargo @args 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE
    $output | ForEach-Object { Write-Host $_ }

    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (source files without a mirrored test)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}

# A tool that reports file-level risk should be held to its own bound. Runs the
# freshly built binary rather than whatever version happens to be installed, so
# the gate reflects the working tree.
#
# The ceiling is a ratchet set just above the current worst file, not at the
# shipped default of 20 — that would leave four times the necessary slack and
# never fire. Lower it when the score improves, never raise it to turn a red
# build green.
#
# Passed as a string, not a [double], so it reaches the CLI as `9.5` on every
# machine; interpolating a [double] formats it with the current culture and
# emits `9,5` on a comma-decimal locale.
function Invoke-Iceberg4RustSelfGate {
    param(
        [string]$Label = "iceberg4rust self-analysis",
        [string]$Threshold
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\Cargo.toml")).Path

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo run --quiet --bin cargo-iceberg4rust -- `
        --manifest-path $manifestPath --threshold $Threshold
    $exitCode = $LASTEXITCODE
    $ErrorActionPreference = $previousErrorActionPreference

    # 2 is the tool's own "offenders found"; anything else non-zero is a failure
    # to run at all.
    if ($exitCode -eq 2) {
        Write-Host "`nFailed: $Label (file at or above the ceiling of $Threshold)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (exit code $exitCode)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}

# ---------------------------------------------------------------------------
# CRAP gate
# ---------------------------------------------------------------------------

Invoke-Crap4RustGate "CRAP iceberg4rust" @("cargo-iceberg4rust")

# ---------------------------------------------------------------------------
# Mirrored test gate
# ---------------------------------------------------------------------------

Invoke-Twin4RustGate "Mirrored tests iceberg4rust" @("cargo-iceberg4rust")

# ---------------------------------------------------------------------------
# File risk gate (self-analysis)
# ---------------------------------------------------------------------------

Invoke-Iceberg4RustSelfGate -Threshold "9.5"

# ---------------------------------------------------------------------------

Write-Host "`niceberg4rust Stage 2 passed!" -ForegroundColor Green
Pop-Location
exit 0
