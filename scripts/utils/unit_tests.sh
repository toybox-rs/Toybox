#!/bin/bash

set -eu

./start_python ../scripts/utils/test_games.py
./start_python toybox/toybox/interventions/amidar.py
./start_python toybox/toybox/interventions/breakout.py
./start_python toybox/toybox/interventions/gridworld.py
./start_python toybox/toybox/interventions/space_invaders.py

