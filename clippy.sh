#!/bin/sh
# TODO should be able to add pedantic clippy allows in Cargo.toml, this is a temporary workaround
cargo clippy --all-targets --all-features -- -Aclippy::range_plus_one "$@"
