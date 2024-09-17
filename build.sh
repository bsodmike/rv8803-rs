#!/bin/bash

set -ex

cargo build --release --no-default-features --features=blocking
cargo test --no-default-features --features=blocking
cargo clippy -- -Dclippy::all -Dclippy::pedantic
cargo doc

# Run on linux
# cargo build --examples --features=linux_blocking --no-default-features
# cargo build --examples --features=linux_async --no-default-features