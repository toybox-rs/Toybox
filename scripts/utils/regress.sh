#!/bin/bash

set -eu

time ./start_python -m baselines.regress --alg=ppo2 --env=AmidarToyboxNoFrameskip-v4 --num_timesteps=0 --load_path ../models/AmidarToyboxNoFrameskip-v4.ppo2.5e7.2959785456.2019-01-15.model --play
time ./start_python -m baselines.regress --alg=ppo2 --env=SpaceInvadersToyboxNoFrameskip-v4 --num_timesteps=0 --load_path ../models/SpaceInvadersToyboxNoFrameskip-v4.ppo2.5e7.3485671521.2019-01-15.model --play
time ./start_python -m baselines.regress --alg=ppo2 --env=BreakoutToyboxNoFrameskip-v4 --num_timesteps=0 --load_path ../models/BreakoutToyboxNoFrameskip-v4.ppo2.5e7.2633929322.2019-01-04.model --play
