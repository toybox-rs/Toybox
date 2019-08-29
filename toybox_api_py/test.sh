#!/bin/bash

set -eu

python -m toybox.interventions.amidar
python -m toybox.interventions.breakout
python -m toybox.interventions.gridworld
python -m toybox.interventions.space_invaders
python test/test_games.py
