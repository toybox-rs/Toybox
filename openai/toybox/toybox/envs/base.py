from abc import ABC, abstractmethod
from gym import Env, error, spaces, utils

class ToyboxBaseEnv(Env, ABC):
    metadata = {'render.modes': ['human']}
    
    def __init__(self, toybox, grayscale=True, alpha=False, actions=None):
        self.toybox = toybox
        self.score = self.toybox.get_score()

        assert(actions is not None)
        self._action_set = actions
        self._obs_type = 'image'
        self._rgba = 1 if grayscale else 4 if alpha else 3
        self._pixel_high = 255

        self._height = self.toybox.get_height()
        self._width = self.toybox.get_width()
        self._dim = (self._height, self._width, self._rgba) 
        
        self.reward_range = (0, float('inf'))
        self.action_space = spaces.Discrete(len(self._action_set))
        self.observation_space = spaces.Box(low=0, high=self._pixel_high, shape=self._dim, dtype='uint8')

    
    @abstractmethod
    def _action_to_input(self, action):
        pass

    def step(self, action_index):
        obs = None
        reward = None
        done = False
        info = {}
    
        assert(type(action_index) == int)
        assert(action_index < len(self._action_set))
    
        # Convert the input action (string or int) into the ctypes struct.
        action = self._action_to_input(self._action_set[action_index])
        obs = self.toybox.apply_action(action)
        
        # Compute the reward from the current score and reset the current score.
        score = self.toybox.get_score()
        reward = max(score - self.score, 0)
        self.score = score
    
        # Check whether the episode is done
        done = self.toybox.game_over()
    
        # Send back dignostic information
        info['lives'] = self.toybox.get_lives()
    
        return obs, reward, done, info

    def reset(self):
        self.toybox.new_game()
        self.score = self.toybox.get_score()
        obs = self.toybox.rstate.render_frame(
        self.toybox.rsimulator, grayscale=self.toybox.grayscale)
        return obs

    def render(self, mode='human', close=False):
        # obs = self.toybox.rstate.render_frame_color(
        #   self.toybox.rsimulator)
        pass

    def close(self):
        del self.toybox
        self.toybox = None
