#!/bin/bash

set -eu

# just to make sure pip installs worked OK!
./scripts/utils/unit_tests.sh 

# regression / with models tests:
if [ ! -e toybox-regress-models.zip ]; then
  wget https://static.jjfoley.me/toybox-regress-models-16-april-2019.zip -O toybox-regress-models.zip
  unzip toybox-regress-models.zip
fi

./scripts/utils/regress.sh
./scripts/utils/unit_tests.sh
./scripts/utils/behavior_tests.sh
