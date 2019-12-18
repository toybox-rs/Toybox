#!/bin/bash

rm -rf target/wheels/*

virtualenv -p python3 check-venv
source check-venv/bin/activate

set -eu

# scripts should do this later for us but frontload on travis...
pip install -r toybox_cffi/requirements.txt
# not explicitly listed elsewhere because you may want tensorflow_gpu instead.
pip install 'tensorflow<2.0'

cargo fmt --all -- --check
cargo test --verbose --all
cargo build --release

if [ ! -e toybox-regress-models.zip ]; then
  wget https://static.jjfoley.me/toybox-regress-models-16-april-2019.zip -O toybox-regress-models.zip
  unzip toybox-regress-models.zip
fi


# install toybox library package locally
cd toybox_cffi && maturin build -b cffi --release && cd - && pip install target/wheels/toybox_cffi*.whl

# run core Toybox API Tests: (includes interventions)
cd toybox_api_py && python3 setup.py install && ./test.sh && cd -

# run Toybox Gym API Tests:
cd toybox_envs_py && python3 setup.py install && ./test.sh && cd -

# required for gym env registration
cd ctoybox && ../scripts/utils/regress.sh

