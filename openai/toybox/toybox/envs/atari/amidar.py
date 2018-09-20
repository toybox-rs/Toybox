from toybox.envs.atari.base import ToyboxBaseEnv
from toybox.envs.atari.constants import *
from toybox.toybox import Toybox, Input
import sys


class AmidarEnv(ToyboxBaseEnv):
    
    def __init__(self, grayscale=True, alpha=False):
        self. _amidar_action_ids = [NOOP_ID, LEFT_ID, RIGHT_ID, UP_ID, DOWN_ID]
        self._amidar_action_strs = [NOOP_STR, LEFT_STR, RIGHT_STR, UP_STR, DOWN_STR]

        super().__init__(Toybox('amidar', grayscale),
            grayscale=grayscale,
            alpha=alpha,
            actions=self._amidar_action_ids)

    def _action_to_input(self, action):
        input = Input()
        action = action if type(action) == str \
                else ACTION_ID_TO_STR_LOOKUP[action]

        if action in self._amidar_action_strs:
            input.set_input(action)
            return input
        else:
            raise ValueError('Unsupported action: %s' % action)