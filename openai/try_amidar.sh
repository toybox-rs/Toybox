NUM_FRAMES=$1

cargo build --release

./start_python toybox_baselines.py --alg=ppo2 --env=toybox-amidar-v0 --num_timesteps=$1 --save_path=$PWD/amidar.ppo2.$1.model
