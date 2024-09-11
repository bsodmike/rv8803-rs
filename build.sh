#!/bin/bash

set -e

cargo build --release
cargo clippy -- -Dclippy::all -Dclippy::pedantic
cargo test
cargo doc
