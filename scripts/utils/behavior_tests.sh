#!/bin/bash

set -eu

# The other tests associated with this class refer to local models
time ./start_python -m unittest toybox.sample_tests.test_amidar.EnemyRemovalTest.test_no_enemmies_ppo2

time ./start_python -m unittest toybox.sample_tests.OneEnemyTargetTest
time ./start_python -m unittest toybox.sample_tests.GangUpNoJumpRandomTest
time ./start_python -m unittest toybox.sample_tests.GangUpNoJumpTargetTest