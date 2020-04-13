#!/bin/bash

set -eu

# The other tests associated with this class refer to local models
time python -m unittest toybox.sample_tests.test_amidar.EnemyRemovalTest.test_no_enemies_ppo2

time python -m unittest toybox.sample_tests.test_amidar.OneEnemyTargetTest
time python -m unittest toybox.sample_tests.test_amidar.GangUpNoJumpRandomTest
time python -m unittest toybox.sample_tests.test_amidar.GangUpNoJumpTargetTest

time python -m unittest toybox.sample_tests.test_breakout.EZChannel
time python -m unittest toybox.sample_tests.test_breakout.LastBrick
time python -m unittest toybox.sample_tests.test_breakout.PolarStarts

time python -m unittest toybox.sample_tests.test_spaceinvaders.NoShields
time python -m unittest toybox.sample_tests.test_spaceinvaders.JitterVary
time python -m unittest toybox.sample_tests.test_spaceinvaders.ShieldXs