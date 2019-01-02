#!/bin/bash

set -eu

# Derived from: https://rustwasm.github.io/wasm-bindgen/whirlwind-tour/basic-usage.html
rustup target add wasm32-unknown-unknown
cargo +nightly install wasm-bindgen-cli

cd html && npm install
