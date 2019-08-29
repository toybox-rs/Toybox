from abc import ABC
from toybox import Toybox

import json
""" Contains the base class for interventions. 

To make interventions for a new game, subclass Intervention."""

class Intervention(ABC):

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


  def set_partial_config(self, fname): 
    import os

    if os.path.isfile(fname): 
      with open(fname) as f:
          data = json.load(f)
          for k in data.keys(): 
            if k in self.config.keys():
              self.config[k] = data[k]
              self.dirty_config = True


  def check_position(self, pdict, key_ls): 
    # check that pdict is a dictionary containing the keys in list ls
    assert isinstance(pdict, dict)
    assert all([k in pdict.keys() for k in key_ls])

    return True


if __name__ == "__main__":
  with Toybox('amidar') as tb:
    state = tb.to_state_json()
    config = tb.config_to_json()
    
    with Intervention(tb, 'amidar') as intervention:
      intervention.config['enemies'] =[]

      new_state = intervention.state
      new_config = intervention.config

    assert len(config['enemies']) == 5
    assert len(new_config['enemies']) == 0
    assert len(tb.config_to_json()['enemies']) == 0


