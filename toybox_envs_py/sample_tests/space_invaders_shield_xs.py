import toybox_envs
from toybox_envs.atari.base import ToyboxBaseEnv
from toybox_envs.atari.amidar import AmidarEnv
from toybox_envs.atari.breakout import BreakoutEnv

import copy
import time
import sys
import csv
import multiprocessing
import os.path as osp
import gym
from collections import defaultdict, Counter
import tensorflow as tf
import numpy as np
from scipy.stats import sem
from statistics import stdev
import json

from baselines.common.vec_env.vec_frame_stack import VecFrameStack
from baselines.common.cmd_util import common_arg_parser, parse_unknown_args, make_vec_env
from baselines.common.tf_util import get_session
from baselines import bench, logger
from importlib import import_module

from baselines.common.vec_env.vec_normalize import VecNormalize
from baselines.common import atari_wrappers, retro_wrappers


# Hot patch atari env so we can get the score
# This is exactly the same, except we put the result of act into the info
from gym.envs.atari import AtariEnv
def hotpatch_step(self, a):
    reward = 0.0
    action = self._action_set[a]
    # Since reward appears to be incremental, dynamically add an instance variable to track.
    # So there's a __getattribute__ function, but no __hasattribute__ function? Bold, Python.
    try:
        self.score = self.score
    except AttributeError:
        self.score = 0.0

    if isinstance(self.frameskip, int):
        num_steps = self.frameskip
    else:
        num_steps = self.np_random.randint(self.frameskip[0], self.frameskip[1])
    
    for _ in range(num_steps):
        reward += self.ale.act(action)
    ob = self._get_obs()
    done = self.ale.game_over()
    # Update score

    score = self.score
    self.score = 0.0 if done else self.score + reward
    # Return score as part of info
    return ob, reward, done, {"ale.lives": self.ale.lives(), "score": score}

AtariEnv.step = hotpatch_step

try:
    from mpi4py import MPI
except ImportError:
    MPI = None

try:
    import pybullet_envs
except ImportError:
    pybullet_envs = None

try:
    import roboschool
except ImportError:
    roboschool = None

_game_envs = defaultdict(set)
for env in gym.envs.registry.all():
    # TODO: solve this with regexes
    env_type = env._entry_point.split(':')[0].split('.')[-1]
    _game_envs[env_type].add(env.id)

# reading benchmark names directly from retro requires
# importing retro here, and for some reason that crashes tensorflow
# in ubuntu
_game_envs['retro'] = {
    'BubbleBobble-Nes',
    'SuperMarioBros-Nes',
    'TwinBee3PokoPokoDaimaou-Nes',
    'SpaceHarrier-Nes',
    'SonicTheHedgehog-Genesis',
    'Vectorman-Genesis',
    'FinalFight-Snes',
    'SpaceInvaders-Snes',
}


def train(args, extra_args):
    env_type, env_id = get_env_type(args.env)
    print('env_type: {}'.format(env_type))

    total_timesteps = int(args.num_timesteps)
    seed = args.seed

    learn = get_learn_function(args.alg)
    alg_kwargs = get_learn_function_defaults(args.alg, env_type)
    alg_kwargs.update(extra_args)

    env = build_env(args)

    if args.network:
        alg_kwargs['network'] = args.network
    else:
        if alg_kwargs.get('network') is None:
            alg_kwargs['network'] = get_default_network(env_type)

    print('Training {} on {}:{} with arguments \n{}'.format(args.alg, env_type, env_id, alg_kwargs))

    model = learn(
        env=env,
        seed=seed,
        total_timesteps=total_timesteps,
        **alg_kwargs
    )

    return model, env


def build_env(args):
    ncpu = multiprocessing.cpu_count()
    if sys.platform == 'darwin': ncpu //= 2
    nenv = args.num_env or ncpu
    alg = args.alg
    rank = MPI.COMM_WORLD.Get_rank() if MPI else 0
    seed = args.seed

    env_type, env_id = get_env_type(args.env)

    if env_type == 'atari':
        if alg == 'acer':
            env = make_vec_env(env_id, env_type, nenv, seed)
        elif alg == 'deepq':
            env = atari_wrappers.make_atari(env_id)
            env.seed(seed)
            env = bench.Monitor(env, logger.get_dir())
            env = atari_wrappers.wrap_deepmind(env, frame_stack=True, scale=True)
        elif alg == 'trpo_mpi':
            env = atari_wrappers.make_atari(env_id)
            env.seed(seed)
            env = bench.Monitor(env, logger.get_dir() and osp.join(logger.get_dir(), str(rank)))
            env = atari_wrappers.wrap_deepmind(env)
            # TODO check if the second seeding is necessary, and eventually remove
            env.seed(seed)
        else:
            frame_stack_size = 4
            env = VecFrameStack(make_vec_env(env_id, env_type, nenv, seed), frame_stack_size)

    return env


def get_env_type(env_id):
    if env_id in _game_envs.keys():
        env_type = env_id
        env_id = [g for g in _game_envs[env_type]][0]
    else:
        env_type = None
        for g, e in _game_envs.items():
            if env_id in e:
                env_type = g
                break
        assert env_type is not None, 'env_id {} is not recognized in env types'.format(env_id, _game_envs.keys())

    return env_type, env_id


def get_default_network(env_type):
    if env_type == 'atari':
        return 'cnn'
    else:
        return 'mlp'

def get_alg_module(alg, submodule=None):
    submodule = submodule or alg
    alg_module = import_module('.'.join(['baselines', alg, submodule]))

    return alg_module


def get_learn_function(alg):
    return get_alg_module(alg).learn


def get_learn_function_defaults(alg, env_type):
    try:
        alg_defaults = get_alg_module(alg, 'defaults')
        kwargs = getattr(alg_defaults, env_type)()
    except (ImportError, AttributeError):
        kwargs = {}
    return kwargs

def parse_cmdline_kwargs(args):
    '''
    convert a list of '='-spaced command-line arguments to a dictionary, evaluating python objects when possible
    '''
    def parse(v):

        assert isinstance(v, str)
        try:
            return eval(v)
        except (NameError, SyntaxError):
            return v

    return {k: parse(v) for k,v in parse_unknown_args(args).items()}


def main():
    # configure logger, disable logging in child MPI processes (with rank > 0)
    arg_parser = common_arg_parser()
    args, unknown_args = arg_parser.parse_known_args()
    extra_args = parse_cmdline_kwargs(unknown_args)

    if MPI is None or MPI.COMM_WORLD.Get_rank() == 0:
        rank = 0
        logger.configure()
    else:
        logger.configure(format_strs=[])
        rank = MPI.COMM_WORLD.Get_rank()

    model, env = train(args, extra_args)
    env.close()


    logger.log("Running trained model")
    env = build_env(args)
    turtle = atari_wrappers.get_turtle(env)
    print(turtle._action_set)
    if not isinstance(turtle, ToyboxBaseEnv): 
            raise ValueError("Not a ToyboxBaseEnv; cannot export state to JSON", turtle)

    n_trials = 30
    max_steps = 5e3
    # get initial config
    config = turtle.toybox.config_to_json()

    shield_configs = [(1,0,0), (0,1,0), (0,0,1), (0,0,0), (1,1,1)]

    dat = [('trained_env', 'trial', 'x', 'count', 's1', 's2', 's3', 'score')]
    with open('space_invaders_shield_xs.tsv', 'w') as fp:
        def run_test(s1,s2,s3):
            obs = env.reset()
            # Plays the game until death or max steps
            for trial in range(n_trials):
                n_steps = 0
                num_lives = turtle.ale.lives()
                done = False
                score = 0

                xs_observed = Counter()
                while n_steps < max_steps and not done:
                    action = model.step(obs)[0]
                    # action = np.random.choice(range(len(turtle._action_set)), 1)[0]
                    num_lives = turtle.ale.lives()
                    obs, _, done, info = env.step(action)    
                    s = info[0]['score']
                    done = done and num_lives == 1
                    if s > score:
                        score = s
                    # count up xs...
                    xs_observed[turtle.toybox.query_state_json('ship_x')] += 1
                    #env.render()
                    #time.sleep(1/30.0)
                    n_steps += 1
                
                obs = env.reset()
                for (x,count) in xs_observed.items():
                    stuff = (extra_args['load_path'], trial, x, count, s1, s2, s3, score)
                    fp.write('{0}\n'.format('\t'.join([str(s) for s in stuff])))
                fp.flush()
                print(xs_observed)

        source_shields = copy.deepcopy(config['shields'])
        for (s1, s2, s3) in shield_configs:
            keep_shields = []
            if s1 == 1:
                keep_shields.append(source_shields[0])
            if s2 == 1:
                keep_shields.append(source_shields[1])
            if s3 == 1:
                keep_shields.append(source_shields[2])
            print((s1,s2,s3,len(keep_shields)))
            config['shields'] = keep_shields
            turtle.toybox.write_config_json(config)
            run_test(s1,s2,s3)

    env.close()

if __name__ == '__main__':
    main()
