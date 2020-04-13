#!/bin/bash

set -eu

time python -m baselines.regress --alg=ppo2 --env=BreakoutToyboxNoFrameskip-v4 --num_timesteps=0 --load_path models/BreakoutToyboxNoFrameskip-v4.regress.model --play
time python -m baselines.regress --alg=ppo2 --env=AmidarToyboxNoFrameskip-v4 --num_timesteps=0 --load_path models/AmidarToyboxNoFrameskip-v4.regress.model --play
time python -m baselines.regress --alg=ppo2 --env=SpaceInvadersToyboxNoFrameskip-v4 --num_timesteps=0 --load_path models/SpaceInvadersToyboxNoFrameskip-v4.regress.model --play
