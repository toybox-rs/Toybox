#!/bin/bash

set -eu

cargo test
cargo fmt --all -- --check

