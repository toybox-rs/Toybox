#!/bin/bash

set -eu

# just to make sure pip installs worked OK!
python -c 'import ctoybox'

if [ ! -e toybox-regress-models.zip ]; then
  wget https://static.jjfoley.me/toybox-regress-models-16-april-2019.zip -O toybox-regress-models.zip
  unzip toybox-regress-models.zip
fi

# required for gym env registration
cd ctoybox && (../scripts/utils/unit_tests.sh && ../scripts/utils/regress.sh && ../scripts/utils/behavior_tests.sh)

