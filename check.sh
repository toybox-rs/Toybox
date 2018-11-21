#!/bin/bash

set -eu

cargo build -p toybox-core
cargo test -p toybox-core
cargo build -p toybox
cargo test -p toybox
cargo build -p ctoybox
cargo test -p ctoybox
cargo fmt --all -- --check

