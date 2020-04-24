#!/bin/bash

set -eu

python scripts/utils/test_games.py
python -m toybox.interventions.amidar
python -m toybox.interventions.breakout
python -m toybox.interventions.space_invaders