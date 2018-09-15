import gym 
import toybox.envs as ai
from gym import error, spaces, utils
from gym.utils import seeding
from openai_shim.toybox import Toybox, Input


class BreakoutEnv(gym.Env):
  metadata = {'render.modes': ['human']}

  def __init__(self, grayscale=True, alpha=False):   
    self._obs_type = 'image'
    self.toybox = Toybox('breakout', grayscale)
    self.score = self.toybox.get_score()

    self._action_set = [ai.NOOP_ID, ai.LEFT_ID, ai.RIGHT_ID]
    self._pixel_high = 100 if grayscale else 255
    self._height = self.toybox.get_height()
    self._width = self.toybox.get_width()
    self._rgba = 1 if grayscale else 4 if alpha else 3
    # Not sure if we need to wrap for grayscale? That's why I set rgba anyway
    self._dim = (self._height, self._width) if grayscale \
      else (self._height, self._width, self._rgba) 

    self.action_space = spaces.Discrete(len(self._action_set))
    self.observation_space = spaces.Box(low=0, high=self._pixel_high, shape=self._dim)
    self.reward_range = (0, float('inf'))


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


  def _action_to_input(self, action):
    input = Input()
    if action == ai.NOOP_STR or action == ai.NOOP_ID:
      return input
    elif action == ai.RIGHT_STR or action == ai.RIGHT_ID:
      input.set_input(ai.RIGHT_STR)
    elif action == ai.LEFT_STR or action == ai.LEFT_ID:
      input.set_input(ai.LEFT_STR)
    else:
      action = action if type(action) == str \
        else ai.ACTION_ID_TO_STR_LOOKUP[action]
      raise ValueError('Unsupported action: %s' % action)
    return input