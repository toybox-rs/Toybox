from toybox.envs.atari.base import ToyboxBaseEnv
from toybox.envs.atari.constants import *
from toybox.toybox import Toybox, Input


class BreakoutEnv(ToyboxBaseEnv):

    def __init__(self, grayscale=True, alpha=False): 
        #self._breakout_action_strs = [NOOP_STR, LEFT_STR, RIGHT_STR, FIRE_STR]
        self._breakout_action_strs = [NOOP_STR, LEFT_STR, RIGHT_STR]
        super().__init__(Toybox('breakout', grayscale), 
            grayscale=grayscale, 
            alpha=alpha, 
            actions=[ACTION_LOOKUP[s] for s in self._breakout_action_strs])

    def _action_to_input(self, action):
        input = Input()
        print("ACTION:", action)
        action = action.upper() if type(action) == str else ACTION_MEANING[action]
        # Remove this later:
        if action == NOOP_STR:
            return input
        elif action == RIGHT_STR:
            input.set_input(RIGHT_STR)
        elif action == LEFT_STR:
            input.set_input(LEFT_STR)
        elif action == FIRE_STR:
            input.set_input(NOOP_STR, button=BUTTON1_STR)
        else:
            raise ValueError('Unsupported action: %s' % action)
        return input

