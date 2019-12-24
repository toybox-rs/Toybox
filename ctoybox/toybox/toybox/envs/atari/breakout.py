from toybox import Toybox, Input
from toybox.envs.atari.base import ToyboxBaseEnv
from toybox.envs.atari.constants import *


class BreakoutEnv(ToyboxBaseEnv):
    def __init__(self, frameskip=(2, 5), repeat_action_probability=0., grayscale=True, alpha=False): 
        super().__init__(Toybox('breakout', grayscale), 
            frameskip, repeat_action_probability,
            grayscale=grayscale, 
            alpha=alpha)

