#!/bin/bash

set -eu

ENVIRONMENT=$1
STEPS=$2
SAVE_TO=$3

time ./start_python -m baselines.run --alg=acktr --seed=42 --env=${ENVIRONMENT} --num_timesteps=${STEPS} --save_path ${SAVE_TO}
