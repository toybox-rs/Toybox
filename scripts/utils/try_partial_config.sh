load_path=./models/AmidarNoFrameskip-v4.ppo2.5e7.845090117.2018-12-29.model
./start_python -m baselines.run --alg=ppo2 --env=AmidarToyboxNoFrameskip-v4 --load_path=$load_path --num_timesteps=0 --num_env=1 --play --show --partial_config=toybox/toybox/interventions/defaults/amidar_quick_config.json
