#!/bin/bash

set -eu

# for CI time, only run 100 frames of each game, twice.
python3 test/benchmark.py 100 2
