cd ctoybox
n=10000
./start_python -m baselines.run --alg=ppo2 --env=AmidarToyboxNoFrameskip-v4 --num_timesteps=$n --save_path=$PWD/amidar$n.model --weights=[0.5,0.5] --seed=11