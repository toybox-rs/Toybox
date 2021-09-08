import gym
from toybox.envs.atari.base import ToyboxBaseEnv


# Get innermost gym.Env (skip all Wrapper)
def get_turtle(env):
  env = env
  while True:
    if isinstance(env, gym.wrappers.time_limit.TimeLimit):
      # Not setting this causes issues later when trying
      # to time step with the TimeLimit wrapper. Not sure how to
      # pass it in.
      # env._max_episode_steps = 1e7
      env = env.env
    elif isinstance(env, ToyboxBaseEnv):
      return env
    elif isinstance(env, gym.Wrapper):
      env = env.env
    elif isinstance(env, gym.Env):
      return env
    else:
      raise ValueError("Can't unwrap", env)


def _reset_deep_kludge(env, timeout):
  env = env
  while True:
    env.reset()
    if isinstance(env, gym.wrappers.time_limit.TimeLimit):
      # Not setting this causes issues later when trying
      # to time step with the TimeLimit wrapper. Not sure how to
      # pass it in.
      env._max_episode_steps = timeout
      env = env.env
    elif isinstance(env, ToyboxBaseEnv):
      return env
    elif isinstance(env, gym.Wrapper):
      env = env.env
    elif isinstance(env, gym.Env):
      return env
    else:
      raise ValueError("Can't unwrap", env)