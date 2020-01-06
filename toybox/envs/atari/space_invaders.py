from toybox import Toybox, Input
from toybox.envs.atari.base import ToyboxBaseEnv
from toybox.envs.atari.constants import *

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
