cd ctoybox
n=5000
./start_python -m baselines.run --alg=ppo2 --env=BreakoutNoFrameskip-v4 --num_timesteps=$n --save_path=$PWD/breakout$n.model --weights=[0.5,0.5] --seed=13