#!/bin/bash

set -ex

cargo build --release --no-default-features --features=blocking
cargo test --no-default-features --features=blocking
cargo clippy -- -Dclippy::all -Dclippy::pedantic
cargo doc
