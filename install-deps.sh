#!/bin/bash

set -eo pipefail

#
# Installs build dependencies;
# Don't run this script directly, instead, use the Makefile target
# This script must be sourced to make npm available
#

if ! command -v cargo >/dev/null 2>&1; then
    if ! command -v rustup >/dev/null 2>&1; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    fi
    rustup update
fi

if ! command -v wasm-pack >/dev/null 2>&1; then
    cargo install wasm-pack
fi

if [ "$DEV" = "1" ] && ! command -v bacon >/dev/null 2>&1; then
    cargo install --locked bacon
fi

set +eo pipefail # Don't pass option through to makefile sourcing this script
