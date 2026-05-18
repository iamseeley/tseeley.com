#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo > /dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --default-toolchain stable
    export PATH="$HOME/.cargo/bin:$PATH"
fi

cargo build --release --no-default-features
./target/release/tseeley build
