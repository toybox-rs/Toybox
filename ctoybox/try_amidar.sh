#!/bin/bash
#
#SBATCH --job-name=try-amidar
#SBATCH --output=foo_%j.out
#SBATCH -e foo_%j.err
NUM_FRAMES=$1

LD_LIBRARY_PATH=gypsum:/home/etosch/toybox/openai/target/release ./start_python toybox_baselines.py --alg=ppo2 --env=toybox-amidar-v0 --num_timesteps=$1 --save_path=$PWD/amidar.ppo2.$1.model
