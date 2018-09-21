from toybox.envs.atari.base import ToyboxBaseEnv
from toybox.envs.atari.constants import *
from toybox.toybox import Toybox, Input


class BreakoutEnv(ToyboxBaseEnv):

    def __init__(self, grayscale=True, alpha=False): 
        super().__init__(Toybox('breakout', grayscale), 
            grayscale=grayscale, 
            alpha=alpha, 
            actions=[NOOP_ID, LEFT_ID, RIGHT_ID])

    def _action_to_input(self, action):
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

