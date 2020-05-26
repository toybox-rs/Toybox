from abc import ABC, abstractmethod
from ctoybox import Toybox
try:
  import ujson as json
except:
  import json

import importlib
import logging
import math
import os
import random
from typing import Union
""" Contains the base class for interventions. 

To make interventions for a new game, subclass Intervention."""


class MutationError(AttributeError):

  def __init__(self, attribute):
    super().__init__('Trying to mutate immutable field %s' % attribute)
    self.attribute = attribute

class Eq(ABC): 

  def __init__(self, obj):
    self.obj = obj
    self.clz = obj.__class__  


class StandardEq(Eq):

  def __eq__(self, other) -> bool:
    for key in self.clz.eq_keys:
      if self.obj.__getattribute__(key) != other.obj.__getattribute__(key):
        return False
    return True


class ProbEq(Eq): 

  def __init__(self, obj):
    super().__init__(obj)
    self.differ = None
    self.key_order = []

  def __repr__(self) -> str:
    return "ProbEq(differ={}, key_order={})".format(self.differ, self.key_order)

  def __eq__(self, other) -> Eq:
    assert type(self) == type(other)
    copy = self.clz.eq_keys[:]
    random.shuffle(copy)

    for key in copy:
      v1 = self.obj.__getattribute__(key) 
      v2 = other.obj.__getattribute__(key)
      assert type(v1) == type(v2), '{} vs {} for {}'.format(type(v1), type(v2), key)

      eq = math.isclose if type(v1) == float else lambda a, b: a == b
      found_differ = lambda x : (x is False) or (isinstance(x, ProbEq) and x.differ is not None)
      with_prefix = lambda x : key + '.' + x 

      if isinstance(v1, Collection):
        indices = list(range(len(v1)))
        random.shuffle(indices)

        if len(v1) != len(v2):
          self.differ = (key, len(v1), len(v2))

        for i in indices:
          cmp = eq(v1[i], v2[i])
          if found_differ(cmp):
            key = '{}[{}].{}'.format(key, i, cmp.differ[0])
            self.differ = (key, cmp.differ[1], cmp.differ[2]) if isinstance(cmp, ProbEq) else (key, v1, v2)
            return self
          else:
            if type(v1) == ProbEq:
              self.key_order.extend(v1.key_order, v2.key_order)

      else:

        cmp = eq(v1, v2)

        if isinstance(cmp, ProbEq):
          if cmp.differ:
            _, a, b = cmp.differ
            self.differ = (with_prefix(cmp.differ[0]), a, b)
            return self
          else: 
            self.key_order.append(key)
        elif cmp is True:
          self.key_order.append(key)
        elif cmp is False:
          self.differ = (key, v1, v2)
          return self
    
    return self

  def __bool__(self):
    return not self.differ


class SetEq(Eq):

  def __init__(self, obj):
    super().__init__(obj)
    self.differs = []

  def __eq__(self, other) -> Eq:
    for key in self.clz.eq_keys:
      v1 = self.obj.__getattribute__(key) 
      v2 = other.obj.__getattribute__(key)
      assert type(v1) == type(v2), '{} vs {} for {}'.format(type(v1), type(v2), key)

      eq = math.isclose if type(v1) == float else lambda a, b: a == b
      with_prefix = lambda x : key + '.' + x 

      if isinstance(v1, Collection):
        indices = list(range(len(v1)))
        random.shuffle(indices)

        if len(v1) != len(v2):
          self.differs = (key, len(v1), len(v2))

        for i in indices:
          cmp = eq(v1[i], v2[i])
          if (isinstance(cmp, SetEq) and len(cmp.differs) > 1) or cmp is False:
            key = '{}[{}].{}'.format(key, i, cmp.differs[0])
            self.differs = (key, cmp.differs[1], cmp.differs[2]) if isinstance(cmp, SetEq) else (key, v1, v2)
            return self

      else:

        cmp = eq(v1, v2)

        if isinstance(cmp, SetEq):
          if cmp.differs:
            self.differs.extend([(with_prefix(t[0]), t[1], t[2]) for t in cmp.differs])
            return self
        elif cmp is False:
          self.differs.append((key, v1, v2))
          return self
    
    return self

  def __bool__(self):
    return len(self.differs) == 0

  def __str__(self):
    return '{' + ';'.join(['({}, {}, {})'.format(*t) for t in self.differs]) + '}'

  def __len__(self):
    return self.differs.__len__()
  

class BaseMixin(ABC):
  """Base class for game objects. Registers mutation so JSON can be pushed via context manager."""

  @classmethod
  @property
  @abstractmethod
  def expected_keys(clz): pass

  @classmethod
  @property
  @abstractmethod
  def eq_keys(clz): pass

  immutable_fields = ['intervention', '_in_init']
  coersions = {}

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
    existing_attrs = self.__dict__.keys()
    adding_new = name not in existing_attrs
    super().__setattr__(name, self.coersions[name](value) if name in self.coersions else value)

    # Prohibit adding fields outside object instantiation/initialization
    if self._in_init or name == '_in_init': return
    assert 'intervention' in existing_attrs
    if name in self.immutable_fields: # and not :
      raise MutationError('Trying mutate immutable field %s' % name)
    if adding_new:
      raise MutationError("Cannot add new field %s to %s" % (name, self.__class__.__name__))
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


  def encode(self, coersions={}):
    dat = {}
    for name, val in vars(self).items():
      if name == 'intervention': continue
      if name == '_in_init': continue
      if name not in self.expected_keys:
        logging.debug('skipping %s in %s; not in expected keys' % (name, type(self).__name__))
        continue
      dat[name] = val.encode() if isinstance(val, BaseMixin) else val
      # Shouldn't need this anymore now that we call coersions in the 
      # overridden __getattribute__
      # if name in self.coersions:
      #   dat[name] = self.coersions[name](dat[name])
    return dat


  def sample(self, *queries):
    if not self.intervention.modelmod:
      print('WARNING: no models for sampling.')
    assert False

  @abstractmethod
  def make_models(self): pass

  def __eq__(self, other) -> Union[bool, Eq]:
    return self.eq_mode(self) == other.eq_mode(other)

  @property
  def eq_mode(self):
    return self.intervention.eq_mode


class Collection(BaseMixin):

  expected_keys = []
  eq_keys = []
  immutable_fields = BaseMixin.immutable_fields + ['coll']

  def __init__(self, intervention, coll, elt_clz):
    super().__init__(intervention)
    self.elt_clz = elt_clz
    self.coll = [elt_clz.decode(intervention, elt, elt_clz) for elt in coll]
    # SAME DEAL AS GAME - THIS SHOULD ALWAYS BE ABSTRACT, HENCE NO RESET OF IN_INIT

  def __eq__(self, other) -> Union[bool, Eq]:
    retval = None
    for i in range(len(self)):
      cmp = self.eq_mode(self[i]) == other.eq_mode(other[i])
      if isinstance(cmp, bool): 
        return cmp
      elif isinstance(cmp, ProbEq):
        # update names on key order
        keys = ['{}[{}].{}'.format(self.elt_clz, i, k) for k in cmp.key_order]
        cmp.key_order = keys
        if retval is None:
          if cmp.differ is None:
            retval = cmp
          else: return cmp
        elif retval.differ is None: 
          retval.key_order.extend(keys)
        else:
          return cmp
      elif isinstance(cmp, SetEq):
        keys = ['{}[{}].{}'.format(self.elt_clz, i, k) for k in cmp.differs]
        cmp.differs = keys   
        if retval is None:
          retval = cmp
        else:
          retval.differs.extend(keys)
    return retval

  def __str__(self):
    return '[{}]'.format(', '.join([str(c) for c in self.coll]))
    
  def __iter__(self): return self.coll.__iter__()

  def __getitem__(self, key): return self.coll.__getitem__(key)

  def __setitem__(self, key, value): 
    self.coll.__setitem__(key, value)
    self.intervention.dirty_state = True
    
  def __len__(self): return self.coll.__len__()

  def append(self, obj):
    assert isinstance(obj, self.elt_clz), '%s must be of type %s' % (obj, self.elt_clz)
    self.coll.append(obj)
    # Since this doesn't trigger the superclass' __setattr__, we need to     sdirty_state manuall
    self.intervention.dirty_state = True

  def extend(self, obj):
    self.coll.extend(obj)
    self.intervention.dirty_state = True

  def insert(self, i, x):
    self.coll.insert(i, x)
    self.intervention.dirty_state = True

  def remove(self, obj):
    self.coll.remove(obj)
    # Since this doesn't trigger the superclass' __setattr__, we need to setdirty_state manuall
    self.intervention.dirty_state = True

  def pop(self, i=-1):
    self.intervention.dirty_state = True
    return self.coll.pop(i)

  def clear(self):
    self.coll.clear()
    self.intervention.dirty_state = True

  def index(self, x, *args):
    return self.coll.index(x, *args)

  def count(self, x):
    return self.coll.count(x)

  def sort(self, key=None, reverse=False):
    self.intervention.dirty_state = True
    self.coll.sort(key=key, reverse=reverse)

  def reverse(self):
    self.intervention.dirty_state = True
    self.coll.reverse()

  def copy(self):
    return Collection(self.intervention, self.coll.copy(), self.elt_clz)
    
  def encode(self):
    return [elt.encode() for elt in self.coll]

  def decode(intervention, coll, clz): 
    return clz(intervention, coll)

  def make_models(self, data): assert False

        
class Intervention(ABC):

  def __init__(self, tb: Toybox, game_name: str, clz: type, modelmod=None, data=None):
    assert tb.game_name == game_name
    self.game_name = game_name
    self.toybox = tb
    self.config = None
    self.dirty_config = False
    self.dirty_state = False
    self.clz = clz
    self.game = None

    self.modelmod = modelmod 
    self.data = data
    self.eq_mode = StandardEq

  def __enter__(self):
    # grab the JSON to be manipulated
    #self.state = self.toybox.to_state_json()
    self.config = self.toybox.config_to_json()
    self.game = self.clz.decode(self, self.toybox.to_state_json(), self.clz)
    if self.modelmod:
      if self.data: self.make_models()
      self.load_models()

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


  def load_models(self):
    return importlib.import_module(self.modelmod, package=__package__)

  def make_models(self): 
    self.clz.make_models(self, self.data)

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


