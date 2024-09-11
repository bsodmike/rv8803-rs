#!/bin/bash

set -ex

cargo build --release
cargo test
cargo clippy -- -Dclippy::all -Dclippy::pedantic
cargo doc
