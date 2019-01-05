from toybox.envs.atari.base import ToyboxBaseEnv
from toybox.envs.atari.constants import *
from toybox.toybox import Toybox, Input

class SpaceInvadersEnv(ToyboxBaseEnv):

    def __init__(self, frameskip=(2, 5), repeat_action_probability=0., grayscale=True, alpha=False):
      tb = Toybox('space_invaders', grayscale)
      actions = tb.get_legal_action_set()
      super().__init__(tb, 
        frameskip,
        repeat_action_probability,
        grayscale=grayscale,
        alpha=alpha,
        actions=actions)

    def _action_to_input(self):
      pass      
