import toybox

import time
import sys
import multiprocessing
import os.path as osp
import gym
from collections import defaultdict
import tensorflow as tf
import numpy as np
from scipy.stats import sem
from statistics import stdev
from PIL import Image

from baselines.common.vec_env.vec_frame_stack import VecFrameStack
from baselines.common.cmd_util import common_arg_parser, parse_unknown_args, make_vec_env
from baselines.common.tf_util import get_session
from baselines import bench, logger
from importlib import import_module

from baselines.common.vec_env.vec_normalize import VecNormalize
from baselines.common import atari_wrappers

_game_envs = defaultdict(set)
for env in gym.envs.registry.all():
    # TODO: solve this with regexes
    env_type = env._entry_point.split(':')[0].split('.')[-1]
    _game_envs[env_type].add(env.id)

def train(args, extra_args):
    env_type, env_id = get_env_type(args.env)
    print('env_type: {}'.format(env_type))
    seed = args.seed

    learn = get_learn_function(args.alg)
    
    alg_kwargs = get_learn_function_defaults(args.alg, env_type)
    alg_kwargs.update(extra_args)
    if 'weights' in alg_kwargs:
        del alg_kwargs['weights']

    env = build_env(args, extra_args)


    if args.network:
        alg_kwargs['network'] = args.network
    else:
        if alg_kwargs.get('network') is None:
            alg_kwargs['network'] = get_default_network(env_type)

    print('Training {} on {}:{} with arguments \n{}'.format(args.alg, env_type, env_id, alg_kwargs))

    model = learn(
        env=env,
        seed=seed,
        total_timesteps=0,
        **alg_kwargs
    )

    return model, env


def build_env(args, extra_args):
    nenv = 1
    alg = args.alg
    seed = args.seed

    env_type, env_id = get_env_type(args.env)

    if env_type == 'atari':
        if alg == 'acer':
            env = make_vec_env(env_id, env_type, nenv, seed)
        elif alg == 'deepq':
            env = atari_wrappers.make_atari(env_id, None)
            env.seed(seed)
            env = bench.Monitor(env, logger.get_dir())
            env = atari_wrappers.wrap_deepmind(env, frame_stack=True)
        else:
            frame_stack_size = 4
            weights = extra_args['weights'] if 'weights' in extra_args else None
            env = VecFrameStack(make_vec_env(env_id, env_type, nenv, seed, weights=weights), frame_stack_size)
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
    arg_parser = common_arg_parser()
    args, unknown_args = arg_parser.parse_known_args()
    extra_args = parse_cmdline_kwargs(unknown_args)

    logger.configure()

    model, env = train(args, extra_args)
    env.close()

    logger.log("Running trained model")
    env = build_env(args, extra_args)
    obs = env.reset()
    turtle = atari_wrappers.get_turtle(env)
    scores = []
    session_scores = set()
    num_games = 0
    # This is a hack to get the starting screen, which throws an error in ALE for amidar
    num_steps = -1

    while num_games < 10:
        actions = model.step(obs)[0]
        num_lives = turtle.ale.lives()
        obs, _, done, info = env.step(actions)
        #done = done and (num_lives == 1 or turtle.ale.game_over())
        #time.sleep(1.0/60.0)
        done = num_lives == 1 and done 
        #done = done.any() if isinstance(done, np.ndarray) else done

        # Make regression testing faster by limiting score.
        # If we earn 500 or so points in any game, we can assume that we've learned something useful.
        if turtle.ale.get_score() > 500:
            done = True

        if isinstance(info, list) or isinstance(info, tuple):
            session_scores.add(np.average([d['score'] for d in info]))
        elif isinstance(info, dict):
            session_scores.add(['score'])
        else:
            session_scores.add(-1)

        if done:
            num_games += 1
            score = max(session_scores)
            scores.append(score)
            session_scores = set()

            print("game %s: %s" % (num_games, score))
            obs = env.reset()
            session_scores = set()


    print("Avg score: %f" % np.average(scores))
    print("Median score: %f" % np.median(scores))
    print("Std error score: %f" % sem(scores))
    print("Std dev score: %f" % stdev(scores))
    env.close()

    # Fail regression test if average is not greater than 100.
    if (np.average(scores) < 50):
        sys.exit(-1)

if __name__ == '__main__':
    main()
