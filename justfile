# Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
# Licensed under the MIT License
# SPDX-License-Identifier: MIT

# just has no POSIX `sh` on PATH on this Windows checkout: Git for Windows'
# sh.exe is installed but not on PATH, and the `bash.exe` that does resolve
# is WSL's, a separate toolchain. PowerShell is the one shell guaranteed
# present without relying on where any particular tool happened to install
# itself. Only recipe bodies are shell-interpreted, so this only matters on
# Windows; ubuntu/macOS runners use just's default `sh` untouched.
set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

# Applies to both stages: `just export` sets it in the process just spawns
# recipe shells from, so it works identically whether that shell turns out
# to be sh or PowerShell -- no per-line, per-platform env-var syntax needed.
export RUSTFLAGS := "-D warnings"

# CI sets fmt to fail on drift instead of silently rewriting files a human
# isn't there to review; a local run still auto-fixes for convenience.
fmt_mode := if env('CI', '') != '' { '--check' } else { '' }

default:
    @just --list

# Formatting, clippy and tests -- cargo built-ins only, so it works on a fresh checkout.
# --workspace on clippy/test so the xtask member is covered too; `cargo fmt`
# formats every workspace member by default already.
stage1:
    cargo fmt {{fmt_mode}}
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace

# House rules, CRAP, mirrored tests and the self-analysis gate, run in that order.
stage2:
    cargo xtask stage2
