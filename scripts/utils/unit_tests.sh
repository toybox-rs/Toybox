#!/bin/bash

set -eu

./start_python scripts/utils/test_games.py
./start_python toybox/interventions/amidar.py
./start_python toybox/interventions/breakout.py
./start_python toybox/interventions/space_invaders.py

