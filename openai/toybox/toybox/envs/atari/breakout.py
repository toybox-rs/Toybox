from toybox.envs.atari.base import ToyboxBaseEnv
from toybox.envs.atari.constants import *
from toybox.toybox import Toybox, Input


class BreakoutEnv(ToyboxBaseEnv):

    def __init__(self, grayscale=True, alpha=False): 
        self._breakout_action_strs = [NOOP_STR, FIRE_STR,  RIGHT_STR, LEFT_STR]
        super().__init__(Toybox('breakout', grayscale), 
            grayscale=grayscale, 
            alpha=alpha, 
            actions=[ACTION_LOOKUP[s] for s in self._breakout_action_strs])

    def _action_to_input(self, action):
        input = Input()
        action = action.upper() if type(action) == str else ACTION_MEANING[action]

        # The easiest way to test new queries is to stream them from here (we're sure it's breakout).
        #print("Bricks_Remaining: ", self.toybox.rstate.breakout_bricks_remaining())
        #print("Channels: ", self.toybox.rstate.breakout_channels())
        #print("Brick_live_left_half: ", sum([self.toybox.rstate.breakout_brick_live_by_index(i) for i in range(54)]))

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

