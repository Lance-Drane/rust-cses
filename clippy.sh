#!/bin/sh
# range_plus_one is ignored because it usually gets compiled into worse assembly
cargo clippy --all-targets --all-features -- -Wclippy::pedantic -Aclippy::range_plus_one "$@"
