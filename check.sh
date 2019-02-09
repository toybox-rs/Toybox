#!/bin/bash

set -eu

cargo fmt --all -- --check
cargo test
cargo build --release

if [ ! -e toybox-regress-models.zip ]; then
  wget https://jjfoley.me/static/toybox-regress-models.zip
  unzip toybox-regress-models.zip
fi

# required for gym env registration
pip3 install gym==0.10.5 atari_py pillow tensorflow opencv-python joblib mpi4py
cd ctoybox && (./start_python test_games.py && ./regress.sh)

