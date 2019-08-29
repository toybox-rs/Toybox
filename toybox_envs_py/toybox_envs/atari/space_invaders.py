from toybox_envs.atari.base import ToyboxBaseEnv
from toybox_envs.atari.constants import *
from toybox import Toybox, Input

class SpaceInvadersEnv(ToyboxBaseEnv):

    def __init__(self, frameskip=(2, 5), repeat_action_probability=0., grayscale=True, alpha=False):
      tb = Toybox('space_invaders', grayscale)
      super().__init__(tb, 
        frameskip,
        repeat_action_probability,
        grayscale=grayscale,
        alpha=alpha)

    def _action_to_input(self):
      pass      
