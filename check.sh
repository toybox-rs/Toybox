#!/bin/bash

set -eu

cargo fmt --all -- --check
cargo test
cargo build --release

# required for gym env registration
pip3 install gym pillow
cd ctoybox && ./start_python test_games.py

