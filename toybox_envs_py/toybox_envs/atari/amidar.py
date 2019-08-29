from toybox_envs.atari.base import ToyboxBaseEnv
from toybox_envs.atari.constants import *
from toybox import Toybox, Input
import sys


class AmidarEnv(ToyboxBaseEnv):

    def __init__(self, frameskip=(2, 5), repeat_action_probability=0., grayscale=True, alpha=False):
        super().__init__(Toybox('amidar', grayscale), 'amidar',
            frameskip, repeat_action_probability,
            grayscale=grayscale,
            alpha=alpha)
