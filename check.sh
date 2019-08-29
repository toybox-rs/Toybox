#!/bin/bash

virtualenv -p python3 check-venv
source check-venv/bin/activate

set -eu

pip install -r ctoybox/REQUIREMENTS.txt
pip install -r toybox_cffi/requirements.txt
pip install tensorflow

cargo fmt --all -- --check
cargo test --verbose --all
cargo build --release

if [ ! -e toybox-regress-models.zip ]; then
  wget https://static.jjfoley.me/toybox-regress-models-16-april-2019.zip -O toybox-regress-models.zip
  unzip toybox-regress-models.zip
fi

# required for gym env registration
cd ctoybox && (../scripts/utils/unit_tests.sh && ../scripts/utils/regress.sh)

