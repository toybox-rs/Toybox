#!/bin/bash

set -eu

# The other tests associated with this class refer to local models
time ./start_python -m unittest toybox.sample_tests.test_amidar.EnemyRemovalTest.test_no_enemies_ppo2

time ./start_python -m unittest toybox.sample_tests.test_amidar.OneEnemyTargetTest
time ./start_python -m unittest toybox.sample_tests.test_amidar.GangUpNoJumpRandomTest
time ./start_python -m unittest toybox.sample_tests.test_amidar.GangUpNoJumpTargetTest

time ./start_python -m unittest toybox.sample_tests.test_breakout.EZChannel
time ./start_python -m unittest toybox.sample_tests.test_breakout.LastBrick
time ./start_python -m unittest toybox.sample_tests.test_breakout.PolarStarts

time ./start_python -m unittest toybox.sample_tests.test_spaceinvaders.NoShields
time ./start_python -m unittest toybox.sample_tests.test_spaceinvaders.JitterVary
time ./start_python -m unittest toybox.sample_tests.test_spaceinvaders.ShieldXs