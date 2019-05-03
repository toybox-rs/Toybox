import toybox
import gym

from collections import Counter

import numpy as np
import cv2
cv2.ocl.setUseOpenCL(False)

from toybox.envs.atari.base import ToyboxBaseEnv
from stable_baselines.common.atari_wrappers import WarpFrame, NoopResetEnv, MaxAndSkipEnv, EpisodicLifeEnv, ScaledFloatFrame, ClipRewardEnv, FireResetEnv, FrameStack
from stable_baselines.common.policies import MlpPolicy
from stable_baselines.common import set_global_seeds
from stable_baselines.common.vec_env import DummyVecEnv, VecFrameStack, SubprocVecEnv
from stable_baselines import PPO2, logger
from stable_baselines.bench import Monitor

from gym import spaces
from gym.wrappers import TimeLimit

# module variables have faster lookup than class or instance vars.
SE_samples = Counter()

def get_complement(env_id):
    if 'Toybox' in env_id:
        return env_id.replace('Toybox', '')
    else:
        game_name, suffix = env_id.split('No')
        return game_name + 'ToyboxNo' + suffix

class SampleEnvs(gym.Wrapper):
    def __init__(self, envs, weights):
        """Alternates between input environments"""
        assert(sum(weights) == 1.0)
        env = np.random.choice(envs, 1)[0]
        turtle = get_turtle(env)
        print('Starting env:', turtle)
        gym.Wrapper.__init__(self, env)
        self.envs = envs
        self.weights = weights
        SE_samples[str(turtle)] += 1
        print('SampleEnvs map', SE_samples)

    def __del__(self):
        print('Samples encountered:\n', SE_samples)

    def reset(self, **kwargs):
        # Takes optional arg for new weights
        if 'weights' in kwargs:
            self.weights = kwargs['weights']
        
        env = np.random.choice(self.envs, p=self.weights)
        print('resetting to env:', env)
        self.env = env
        gym.Wrapper.__init__(self, env)
        self.env.reset(**kwargs)
        SE_samples[str(get_turtle(env))] += 1 
        obs, _, _, _ = self.env.step(0)
        return obs

    def step(self, action):
        res = self.env.step(action)
        info = res[-1]
        info['samples'] = SE_samples
        return res


# Get innermost gym.Env (skip all Wrapper)
def get_turtle(env):
    env = env
    while True:
        if (isinstance(env, VecFrameStack)):
            env = env.venv
        elif (isinstance(env, gym.Wrapper)):
            env = env.env
        elif (isinstance(env, DummyVecEnv)):
            env = env.envs[0]
        elif isinstance(env, ToyboxBaseEnv):
            return env
        elif isinstance(env, SubprocVecEnv):
            env = env.example_env 
        elif isinstance(env, gym.Env):
            return env
        else:
            raise ValueError("Can't unwrap", env)

def make_wrapper(env_id):
    if 'Toybox' in env_id:
        env = TimeLimit(gym.make(env_id))
    else:
        env= gym.make(env_id)
    assert 'NoFrameskip' in env.spec.id
    env = NoopResetEnv(env, noop_max=30)
    env = MaxAndSkipEnv(env, skip=4)
    return env

class ToyboxWarpFrame(gym.ObservationWrapper):
    def __init__(self, env):
        """Warp frames to 84x84 as done in the Nature paper and later work."""
        gym.ObservationWrapper.__init__(self, env)
        self.width = 84
        self.height = 84
        self.observation_space = spaces.Box(low=0, high=255,
            shape=(self.height, self.width, 1), dtype=np.uint8)
    def observation(self, frame):
        frame = cv2.resize(frame, (self.width, self.height), interpolation=cv2.INTER_AREA)
        return frame[:, :, None]

def wrap_deepmind(env, env_id, episode_life=True, clip_rewards=True, frame_stack=False, scale=False):
    """Configure environment for DeepMind-style Atari.
    """
    if episode_life:
        env = EpisodicLifeEnv(env)
    if 'FIRE' in env.unwrapped.get_action_meanings():
        env = FireResetEnv(env)
    if 'Toybox' in env_id:
        env = ToyboxWarpFrame(env)
    else:
        env = WarpFrame(env)
    if scale:
        env = ScaledFloatFrame(env)
    if clip_rewards:
        env = ClipRewardEnv(env)
    if frame_stack:
        env = FrameStack(env, 4)
    return env


def make_atari_or_toybox(env_id, sample_weights):
    env = make_wrapper(env_id)
    if sample_weights:
        env1 = env
        env2 = make_wrapper(get_complement(env_id))
        return SampleEnvs([env1, env2], sample_weights)
    return env

def make_toybox_or_atari_env(log_dir, env_id, env_type, num_env, seed, weights=[], wrapper_kwargs=None, start_index=0, reward_scale=1.0):
    """
    Create a wrapped, monitored SubprocVecEnv for Atari and MuJoCo.
    """
    if wrapper_kwargs is None: wrapper_kwargs = {}
    def make_env(rank): # pylint: disable=C0111
        def _thunk():
            env = make_atari_or_toybox(env_id, weights)
            env.seed(seed + rank if seed is not None else None)
            env = Monitor(env, log_dir, allow_early_resets=True)
            return wrap_deepmind(env, env_id, **wrapper_kwargs)
        return _thunk
    set_global_seeds(seed)
    if num_env > 1: return SubprocVecEnv([make_env(i + start_index) for i in range(num_env)])
    else: return DummyVecEnv([make_env(start_index)])

