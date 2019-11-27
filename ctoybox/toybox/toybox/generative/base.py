from abc import ABC
from toybox.toybox import Toybox

import json
""" Contains the base class for Roguelike generative helper functions. 

To make modifications to randomize or procedurally generate aspects of game state, subclass Generative."""

class Generative(ABC):

  def __init__(self, tb, game_name):
    # check that the simulation in tb matches the game name.
    self.toybox = tb
    self.state = None
    self.config = None
    self.dirty_config = False
    self.game_name = game_name

    assert tb.game_name == game_name


  def __enter__(self):
    # grab the JSON to be manipulated
    self.state = self.toybox.to_state_json()
    self.config = self.toybox.config_to_json()

    return self

  def __exit__(self, exec_type, exc_value, traceback):
    # commit the JSON

    self.toybox.write_config_json(self.config)
    if self.dirty_config: 
      self.toybox.new_game()
    else: 
      self.toybox.write_state_json(self.state)

    self.state = None
    self.config = None


  def set_partial_config(self, data): 
    pass 



if __name__ == "__main__":
  with Toybox('amidar') as tb:
    state = tb.to_state_json()
    config = tb.config_to_json()
    
    with Generative(tb, 'amidar') as rogue:
      rogue.config['enemies'] =[]

      new_state = intervention.state
      new_config = intervention.config

    assert len(config['enemies']) == 5
    assert len(new_config['enemies']) == 0
    assert len(tb.config_to_json()['enemies']) == 0


