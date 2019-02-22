from abc import ABC
""" Contains the base class for interventions. 

To make interventions for a new game, subclass Intervention."""

class Intervention(ABC):

  def __init__(self, tb, game_name):
    # check that the simulation in tb matches the game name.
    self.toybox = tb
    self.json = tb.get_json()

  def __enter__(self):
    # grab the JSON to be manipulated
    pass

  def __exit__(self, exec_type, exc_value, traceback):
    # commit the JSON
    pass