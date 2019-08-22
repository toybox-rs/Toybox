import toybox
from toybox.envs.atari.base import ToyboxBaseEnv
from toybox.envs.atari.amidar import AmidarEnv
from toybox.envs.atari.breakout import BreakoutEnv
import gym

import multiprocessing
import sys

from baselines.common.vec_env.vec_frame_stack import VecFrameStack
from baselines.common.cmd_util import make_vec_env
from baselines.common.atari_wrappers import DummyVecEnv, SubprocVecEnv


# This is necessary to use the openai baselines/gym combo
# They aren't cleanly separated for 

# Hot patch atari env so we can get the score
# This is exactly the same, except we put the result of act into the info
from gym.envs.atari import AtariEnv
def _hotpatch_step(self, a):
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

AtariEnv.step = _hotpatch_step


# Get innermost gym.Env (skip all Wrapper)
def _get_turtle(env):
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


def setUpToybox(testclass, env_id, seed):
    nenv = 1
    frame_stack_size = 4
    env_type = 'atari'
  
    # Nb: OpenAI special cases acer, trpo, and deepQ.
    env = VecFrameStack(make_vec_env(env_id, env_type, nenv, seed) , frame_stack_size)
  
    testclass.env = env
    testclass.turtle = _get_turtle(env)

def tearDownToybox(testclass):
    testclass.env.close()

def runTest(testclass):
    dat = [('trained_env', 'trial', 'step', 'mvmt', 'score')]
    def add_dat(env=None, trial=None, step=None, mvmt=None, score=None):
        assert (env and trial and step and mvmt and score)
        dat.append((env, trial, step, mvmt, score))

    for trial in range(testclass.trials):
      # for each trial, record the score at mod 10 steps 
      n_steps = 0
      num_lives = testclass.turtle.ale.lives()
      done = False
      while n_steps < testclass.timeout and not done:
        action = testclass.takeAction()
        num_lives = testclass.turtle.ale.lives()                
        obs, _, done, info = testclass.env.step(action)
        testclass.obs = obs
        testclass.env.render()
        import time
        time.sleep(1/30.0)
        done = done.all() and num_lives == 1
        score = info[0]['score']
        if n_steps % testclass.record_period == 0:
          # d = (extra_args['load_path'], trial, n_steps, prot, score)
          # print("{}\t{}\t{}\t{}\t{}".format(*d))
          # dat.append(d)
          n_steps += 1
          #('trained_env', 'trial', 'step', 'mvmt', 'score')
          testclass.obs = testclass.env.reset()
          if testclass.reset_config:
            testclass.turtle.toybox.write_config_json(testclass.reset_config)

    with open('amidar_no_enemies_{}.tsv'.format(extra_args['load_path']), 'w') as fp:
        for row in dat:
            fp.write("{}\t{}\t{}\t{}\t{}\n".format(*row))

    
