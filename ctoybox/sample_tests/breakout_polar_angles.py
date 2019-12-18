import toybox_envs

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
import math
from tqdm import tqdm

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
    obs = env.reset()
    turtle = atari_wrappers.get_turtle(env)

    data = []


    # get initial state
    start_state = turtle.toybox.to_json()
    config = turtle.toybox.config_to_json()
    ball_speed_slow = config['ball_speed_slow']

    # Only let them beat one level.
    clear_level_score = sum([b['points'] for b in start_state['bricks']])

    # Give the ball only one life.
    start_state['lives'] = 1

    ball = start_state['balls'][0]
    # Use center ball position.
    ball['position']['x'] = 120.0
    ball['position']['y'] = 80.0

    #for angle in [90,90,270,270]:
    for angle in range(0,360,5):
        velocity_y = ball_speed_slow * math.sin(math.radians(angle))
        velocity_x = ball_speed_slow * math.cos(math.radians(angle))

        if abs(velocity_y) < 0.0001:
            print('Skip angle=', angle, 'velocity_y=', velocity_y)
            continue
        ball['velocity']['y'] = velocity_y
        ball['velocity']['x'] = velocity_x

        # indicate we're starting a new angle
        print(angle, velocity_x, velocity_y)

        for trial in range(30):
            time.sleep(1/60.0);
            obs = env.reset()
            time.sleep(1/60.0);

            # overwrite state inside the env wrappers:
            turtle.toybox.write_json(start_state)
            # Take a step to overwrite anything that's stuck there, e.g., gameover
            obs, _, done, info = env.step(0)

            # keep track of the score as best in case a game_over wipes it out while we're reading
            best_score = 0

            tup = None
            # give it 2 minutes of human time to finish.
            for t in range(7200):
                actions = model.step(obs)[0]
                obs, _, done, info = env.step(actions)
                score = turtle.toybox.get_score()
                if score > best_score:
                    best_score = score
                tup = (trial, angle, velocity_x, velocity_y, best_score, t)
                if done or turtle.toybox.get_lives() != 1 or turtle.toybox.game_over() or score >= clear_level_score:
                    break

            # how did we do?
            print(tup)
            data.append(tup)
    
    with open('polar_angles2.tsv', 'w') as fp:
        for row in data:
            print('\t'.join([str(r) for r in row]), file=fp)

    env.close()

if __name__ == '__main__':
    main()
