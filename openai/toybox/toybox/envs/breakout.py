import gym 
from gym import error, spaces, utils
from gym.utils import seeding
from toybox.envs.constants import *
from toybox.envs.base import ToyboxBaseEnv
from toybox.toybox import Toybox, Input


class BreakoutEnv(ToyboxBaseEnv):
  metadata = {'render.modes': ['human']}

  def __init__(self, grayscale=True, alpha=False):       
    super().__init__(Toybox('breakout', grayscale), grayscale=True, alpha=False)
    self._action_set = [NOOP_ID, LEFT_ID, RIGHT_ID]

  def action_to_input(self, action):
    input = Input()
    if action == NOOP_STR or action == NOOP_ID:
      return input
    elif action == RIGHT_STR or action == RIGHT_ID:
      input.set_input(RIGHT_STR)
    elif action == LEFT_STR or action == LEFT_ID:
      input.set_input(LEFT_STR)
    else:
      action = action if type(action) == str \
        else ACTION_ID_TO_STR_LOOKUP[action]
      raise ValueError('Unsupported action: %s' % action)
    return input