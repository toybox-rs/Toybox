from stable_baselines.common.policies import MlpPolicy, CnnPolicy
from stable_baselines.common import set_global_seeds
from stable_baselines.common.vec_env import DummyVecEnv, VecFrameStack, SubprocVecEnv
from stable_baselines import PPO2, ACER, A2C, DQN, ACKTR, logger
from stable_baselines.results_plotter import load_results, ts2xy
from gym_helpers import make_toybox_or_atari_env

import argparse
import multiprocessing
import datetime
import os

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Toybox or Atari training with Stable Baselines.')
    parser.add_argument('env', type=str, help='e.g., AmidarToyboxNoFrameskip-v4')
    parser.add_argument('--num_env', type=int, default=multiprocessing.cpu_count())
    parser.add_argument('--seed', type=int, default=42)
    parser.add_argument('--algorithm', type=str, default='ppo2')
    parser.add_argument('--policy', type=str, default='cnn')
    parser.add_argument('--num_timesteps', type=float, default=int(1e5))
    parser.add_argument('--save_path', type=str, default=None)
    parser.add_argument('--verbose', type=int, default=1)
    parser.add_argument('--load_path', type=str, default=None)
    parser.add_argument('--play', type=int, default=0)
    parser.add_argument('--render', default=False, action='store_true')
    parser.add_argument('--log_dir_name', type=str, default=None)
    parser.add_argument('--save_interval', type=int, default=1_000_000)

    args = parser.parse_args()
    num_timesteps = int(args.num_timesteps)

    # This is a very opinionated 
    log_dir_name = '%s_%s_%s_%d_%s' % (args.env, args.algorithm, args.policy, args.seed, datetime.datetime.now().isoformat())
    os.makedirs(log_dir_name, exist_ok=False)

    set_global_seeds(args.seed)
    env = make_toybox_or_atari_env(log_dir_name, args.env, 'atari', num_env=args.num_env, seed=args.seed)

    policy_fn = None
    model_fn = None

    if args.policy == 'cnn':
        policy_fn = CnnPolicy
    elif args.policy == 'mlp':
        policy_fn = MlpPolicy
    else:
        raise 'Not-yet-supported policy: '+args.policy

    alg = args.algorithm.lower()
    model_fn = None
    if alg == 'ppo2':
        model_fn = PPO2
    elif alg == 'acer':
        model_fn = ACER
    elif alg == 'acktr':
        model_fn = ACKTR
    elif alg == 'a2c':
        model_fn = A2C
    elif alg == 'dqn':
        model_fn = DQN
    else:
        raise 'Not-yet-supported model algorithm: '+args.algorithm

    model = model_fn(policy_fn, env, verbose=args.verbose)
    if args.load_path:
        model.load(args.load_path)

    # Save roughly every save_interval with a callback:
    last_save = 0
    def update_callback(_locals, _globals):
        global last_save
        timestep = model.num_timesteps
        if timestep - last_save > args.save_interval:
            model.save(os.path.join(log_dir_name, 't%d.model' % timestep))
            last_save = timestep

    model.learn(total_timesteps=num_timesteps, callback=update_callback)

    # always save the last model...
    model.save(os.path.join(log_dir_name, 't%d.model' % num_timesteps))

    if args.save_path:
        model.save(args.save_path)

    if args.play > 0:
        obs = env.reset()
        for i in range(args.play):
            action, _states = model.predict(obs)
            obs, rewards, dones, info = env.step(action)
            if args.render:
                env.render()

