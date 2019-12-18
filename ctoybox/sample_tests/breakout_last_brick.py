import toybox_envs
from toybox_envs.atari.base import ToyboxBaseEnv
from toybox_envs.atari.amidar import AmidarEnv
from toybox_envs.atari.breakout import BreakoutEnv

import time
import sys
import csv
import matplotlib as mpl
import matplotlib.cm as cm
import multiprocessing
import os.path as osp
import gym
from collections import defaultdict
import tensorflow as tf
import numpy as np
from scipy.stats import sem
from statistics import stdev

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

# from https://stackoverflow.com/questions/40948069/color-range-python
def convert_to_rgb(minimum, maximum, value):
    norm = mpl.colors.Normalize(vmin=minimum, vmax=maximum)
    cmap = cm.hot

    m = cm.ScalarMappable(norm=norm, cmap=cmap)
    print(m.to_rgba(value))

    norm = (value - minimum)/(maximum - minimum)
    (r, g, b) = colorsys.hsv_to_rgb(norm, 1.0, 1.0)
    R, G, B = int(255 * r), int(255 * g), int(255 * b)

    return R, G, B

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
    obs = env.reset()
    turtle = atari_wrappers.get_turtle(env)
    if not isinstance(turtle, ToyboxBaseEnv): 
            raise ValueError("Not a ToyboxBaseEnv; cannot export state to JSON", turtle)

    # get total number of bricks
    n_bricks = turtle.toybox.rstate.breakout_bricks_remaining()

    n_trials = 30
    #brick_info = [["brick_id", "avg_steps", "median_steps", "std_dev_steps", "n_successes"]]
    brick_info = []

    # get initial state
    start_state = turtle.toybox.to_json()

    # for each brick, remove and run n_trials to determine number of steps until success (or death)
    for brick in range(n_bricks): 
        # remove all bricks
        for b in range(n_bricks): 
            start_state["bricks"][b]["alive"] = False

        # turn on single brick
        start_state["bricks"][brick]["alive"] = True
        # get bricks core
        brick_score = start_state["bricks"][brick]["points"]

        # load env from manipulated state
        turtle.toybox.write_json(start_state)

        # n trials 
        num_games = 0
        n_steps = 0
        step_counts = []
        successes = 0

        # stop game after 4 minutes of human gameplay
        max_steps = 7200

        while num_games < n_trials:
            actions = model.step(obs)[0]
            n_steps += 1

            num_lives = turtle.ale.lives()
            obs, _, done, info = env.step(actions)
            #env.render()

            bricks_remaining = info[0]['score'] < brick_score       
            done = (num_lives == 1 and done) or not bricks_remaining

            if done or n_steps > 7200:
                num_games += 1

                obs = env.reset()
                turtle.toybox.write_json(start_state)

                step_counts.append(n_steps)
                n_steps = 0

                if not bricks_remaining: 
                    successes += 1

        avg = np.average(step_counts)
        med = np.median(step_counts)
        print("brick %s: avg %s steps, %s median steps, %s/%s successes" % (brick, avg, med, successes, n_trials))
        brick_info.append([brick, avg, med, stdev(step_counts), successes])

        with open('last_brick.tsv', 'w') as fp:
            for row in brick_info:
                print('\t'.join([str(x) for x in row]), file=fp)



   # create image from brick clearing performance in steps
    # for brick in range(n_bricks): 
    #     start_state["bricks"][brick]["alive"] = True
    #     r,g,b = rgb(0,max,value)
    #     start_state["bricks"][brick]["color"]["r"] = r
    #     start_state["bricks"][brick]["color"]["g"] = g
    #     start_state["bricks"][brick]["color"]["b"] = b

    env.close()

if __name__ == '__main__':
    main()
