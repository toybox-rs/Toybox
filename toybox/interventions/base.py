from abc import ABC, abstractmethod
import inspect
from ctoybox import Toybox
try:
  import ujson as json
except:
  import json
""" Contains the base class for interventions. 

To make interventions for a new game, subclass Intervention."""

class BaseMixin(ABC):
  """Base class for game objects. Registers mutation so JSON can be pushed via context manager."""

  @classmethod
  @property
  @abstractmethod
  def expected_keys(clz): pass

  @classmethod
  @property
  @abstractmethod
  def immutable_fields(clz): pass

  def __init__(self, intervention):
    self._in_init = True
    self.intervention = intervention


  # Refactor notes 4/17/2020 (EMT)
  # Inspecting the call stack was causing major overhead:
  #
  # 15278049 function calls (15277560 primitive calls) in 7.386 seconds
  # removing the call to inspect:
  # 10338 function calls (9903 primitive calls) in 0.011 seconds
  #
  # Unfortunately, the workaround requires some vigilance. Rather than inspecting
  # the call stack to see if we are currently in the __init__ function (where we
  # are allowed to set fields without writing to toybox), we are tracking whether
  # we are current in __init__ via inheritence and the manual update of a flag in 
  # the children's __init__ functions. 
  def __setattr__(self, name, value):
    # stack = [frame.function for frame in inspect.stack()]
    # calling_fn = stack[1]
    existing_attrs = self.__dict__.keys()
    adding_new = name not in existing_attrs
    super().__setattr__(name, value)
    # Prohibit adding fields outside object instantiation/initialization
    # if calling_fn == '__init__': return 
    if self._in_init or name == '_in_init': return
    assert 'intervention' in existing_attrs
    if name in self.immutable_fields: # and not :
      raise AttributeError('Trying mutate immutable field %s' % name)
    if adding_new:
      raise AttributeError("Cannot add new field %s to %s" % (name, self.__class__.__name__))
    self.intervention.dirty_state = True
    
  
  def decode(intervention, obj, clz):
    """Creates an instance of the input class from the JSON. 
    
    All game elements inherit from BaseMixin. `decode` should be called recursively. 

    Parameters
    ---
    intervention : Intervention
      The context manager
    obj : json
      The input JSON blob
    clz : Class
      The subclass being instantiated

    Returns
    ---
    BaseMixin
      A subclass of BaseMixin corresponding to a game or game element. 
    
    """
    actual_keys = set(obj.keys()) 
    expected_keys = set(clz.expected_keys)
    target_name = clz.__name__

    intersection = actual_keys.intersection(expected_keys)
    not_enough = len(expected_keys) > len(intersection)
    too_many = len(actual_keys) > len(intersection)

    if not_enough:
      raise ValueError("Missing keys (%s); maybe input is not a %s object?" % (
        str(expected_keys.difference(actual_keys)), target_name))

    elif too_many:
      raise ValueError("Input object contains too many keys (%s); has the specification for %s changed?" % (
        str(actual_keys), target_name))

    else: return clz(intervention, **obj)        


  def encode(self):
    dat = {}
    for name, val in vars(self).items():
      if name == 'intervention': continue
      if name == '_in_init': continue
      if name not in self.expected_keys:
        if __debug__:
          print('skipping %s in %s; not in expected keys' % (name, type(self).__name__))
        continue
      dat[name] = val.encode() if isinstance(val, BaseMixin) else val
    return dat
        
class Intervention(ABC):

  def __init__(self, tb: Toybox, game_name: str, clz: type):
    self.toybox = tb
    self.config = None
    self.dirty_config = False
    self.dirty_state = False
    self.game_name = game_name
    assert tb.game_name == game_name
    self.clz = clz
    self.game = None

  def __enter__(self):
    # grab the JSON to be manipulated
    #self.state = self.toybox.to_state_json()
    self.config = self.toybox.config_to_json()
    self.game = self.clz.decode(self, self.toybox.to_state_json(), self.clz)

    return self

  def __exit__(self, exec_type, exc_value, traceback):
    # commit the JSON
    
    if self.dirty_config:
      self.toybox.write_config_json(self.config)
      self.toybox.new_game()

    elif self.dirty_state:
      self.toybox.write_state_json(self.game.encode())

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


if __name__ == "__main__":
  with Toybox('amidar') as tb:
    state = tb.to_state_json()
    config = tb.config_to_json()
    
    with Intervention(tb, 'amidar') as intervention:
      intervention.config['enemies'] = []

      new_state = intervention.state
      new_config = intervention.config

    assert len(config['enemies']) == 5
    assert len(new_config['enemies']) == 0
    assert len(tb.config_to_json()['enemies']) == 0


