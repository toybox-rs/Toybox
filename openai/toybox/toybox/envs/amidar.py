from toybox.envs.base import ToyboxBaseEnv
from toybox.envs.constants import *
from toybox.toybox import Toybox, Input


class AmidarEnv(ToyboxBaseEnv):

    _action_ids = [NOOP_ID, LEFT_ID, RIGHT_ID, UP_ID, DOWN_ID]
    _action_strs = [NOOP_STR, LEFT_STR, RIGHT_STR, UP_STR, DOWN_STR]

    
    def __init__(self, grayscale=True, alpha=False):
        super().__init__(Toybox('breakout', grayscale),
            grayscale=grayscale,
            alpha=alpha,
            actions=_action_ids)

    def _action_to_input(self, action):
        input = Input()
        if action in _action_ids:
            input.set_input(ACTION_ID_TO_STR_LOOKUP[action])
            return input
        elif action in _action_strs:
            input.set_input(action)
            return input
        else:
            action = action if type(action) = str \
                else ACTION_ID_TO_STR_LOOKUP[action]
            raise ValueError('Unsupported action: %s' % action)