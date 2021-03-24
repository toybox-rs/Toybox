#!/bin/bash

set -eu

python scripts/utils/test_games.py
#python -m toybox.interventions.amidar
#python -m toybox.interventions.breakout
python -m toybox.interventions.space_invaders

# unittest will discover test.interventions.test_amidar_interventions
# and test.interventions.test_breakout_interventions
# so we only request toybox.interventions.space_invaders above
python -m unittest discover test.interventions
