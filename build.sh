#!/bin/bash

set -ex

cargo build --release
cargo test
cargo clippy -- -D warnings
cargo doc
