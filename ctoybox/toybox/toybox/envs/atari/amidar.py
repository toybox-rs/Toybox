from toybox.envs.atari.base import ToyboxBaseEnv
from toybox.envs.atari.constants import *
from toybox.toybox import Toybox, Input
import sys


class AmidarEnv(ToyboxBaseEnv):

    def __init__(self, frameskip=(2, 5), repeat_action_probability=0., grayscale=True, alpha=False):
        self._amidar_action_strs = [
            NOOP_STR, FIRE_STR, LEFT_STR, RIGHT_STR, UP_STR, DOWN_STR,
            UPFIRE_STR, DOWNFIRE_STR, LEFTFIRE_STR, RIGHTFIRE_STR
        ]
        self. _amidar_action_ids = [ACTION_LOOKUP[s] for s in self._amidar_action_strs]

        super().__init__(Toybox('amidar', grayscale), 'amidar',
            frameskip, repeat_action_probability,
            grayscale=grayscale,
            alpha=alpha,
            actions=self._amidar_action_ids)

    def _action_to_input(self, action):
        input = Input()
        action = action.upper() if type(action) == str \
                else ACTION_MEANING[action]

        #state = self.toybox.rstate
        #print("tiles_to_paint", state.amidar_num_tiles_unpainted())
        #print("player", state.amidar_player_tile(), "regular", state.amidar_regular_mode())
        #print("jumps", state.amidar_jumps_remaining())
        #print("enemies", state.amidar_enemy_tiles())

        if action in self._amidar_action_strs:
            # Action ids < 6 are atomic actions
            if action == FIRE_STR:
                input.set_input(NOOP_STR, button=BUTTON1_STR)
            elif ACTION_LOOKUP[action] < 6:
                input.set_input(action)
            else:
                # All of the valid compound inputs for amidar end in "FIRE"
                input.set_input(action[:-4], button=BUTTON1_STR)
            return input
        else:
            raise ValueError('Unsupported action: %s' % action)

