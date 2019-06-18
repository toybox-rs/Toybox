from toybox.envs.atari.breakout import BreakoutEnv
from toybox.envs.atari.constants import *
from toybox.toybox import Toybox, Input


class BreakoutGenEnv(BreakoutEnv):

    def __init__(self, frameskip=(2, 5), repeat_action_probability=0., grayscale=True, alpha=False): 
        super().__init__(Toybox('breakout', grayscale), 
            frameskip, repeat_action_probability,
            grayscale=grayscale, 
            alpha=alpha)