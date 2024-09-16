#!/bin/bash

set -ex

cargo build --release --no-default-features --features=defmt,async
cargo build --release --no-default-features --features=defmt,blocking
cargo clippy -- -Dclippy::all -Dclippy::pedantic
cargo test --no-default-features --features=defmt,async
cargo test --no-default-features --features=defmt,blocking
cargo doc
