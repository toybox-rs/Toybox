from abc import ABC, abstractmethod
from gym import Env, error, spaces, utils

import numpy as np

# LazyFrames taken from OpenAI Baselines
# https://github.com/openai/baselines/blob/master/baselines/common/atari_wrappers.py
# commit 8c2aea2addc9f3ba36d4a0c937e6a2d09830afc7
class LazyFrames(object):
    def __init__(self, frames):
        self._frames = frames
        self._out = None

    def _force(self):
        if self._out is None:
            self._out = np.concatenate(self._frames, axis=2)
            self._frames = None
        return self._out

    def __array__(self, dtype='uint8'):
        out = self._force()
        if dtype is not None:
            out = out.astype(dtype)
        return out

    def __len__(self):
        return len(self._force())

    def __getitem__(self, i):
        return self._force()[i]


class ToyboxBaseEnv(Env, ABC):
    metadata = {'render.modes': ['human']}
    
    def __init__(self, toybox, grayscale=True, alpha=False, actions=None):
        assert(toybox.state)
        self.toybox = toybox
        self.score = self.toybox.get_score()

        assert(actions is not None)
        self._action_set = actions
        self._obs_type = 'image'
        self._rgba = 1 if grayscale else 4 if alpha else 3
        self._pixel_high = 255

        self._height = self.toybox.get_height()
        self._width = self.toybox.get_width()
        self._dim = (self._height, self._width, self._rgba * len(self.toybox.get_state())) 
        
        self.reward_range = (0, float('inf'))
        self.action_space = spaces.Discrete(len(self._action_set))
        self.observation_space = spaces.Box(
            low=0, 
            high=self._pixel_high, 
            shape=self._dim, 
            dtype='uint8')
    
    @abstractmethod
    def _action_to_input(self, action):
        raise NotImplementedError

    # From OpenAI Gym Baselines
    # https://github.com/openai/baselines/blob/master/baselines/common/atari_wrappers.py
    def _get_obs(self):
        assert len(self.toybox.state) == self.toybox.k
        return LazyFrames(list(self.toybox.get_state()))

    def step(self, action_index):
        obs = None
        reward = None
        done = False
        info = {}
    
        assert(self.toybox.state)
        assert(type(action_index) == int)
        assert(action_index < len(self._action_set))
    
        # Convert the input action (string or int) into the ctypes struct.
        action = self._action_to_input(self._action_set[action_index])
        frame = self.toybox.apply_action(action)
        obs = self._get_obs()
        
        # Compute the reward from the current score and reset the current score.
        score = self.toybox.get_score()
        reward = max(score - self.score, 0)
        self.score = score
    
        # Check whether the episode is done
        done = self.toybox.game_over()
    
        # Send back dignostic information
        info['lives'] = self.toybox.get_lives()
        info['frame'] = frame
    
        return obs, reward, done, info

    def reset(self):
        self.toybox.new_game()
        self.score = self.toybox.get_score()
        obs = self._get_obs()
        return obs

    def render(self, mode='human', close=False):
        # obs = self.toybox.rstate.render_frame_color(
        #   self.toybox.rsimulator)
        pass

    def close(self):
        del self.toybox
        self.toybox = None