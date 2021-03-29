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


class InterventionNoneError(AttributeError):

  def __init__(self):
    super().__init__('intervention cannot be None')

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

  def _float_eq(this, that):
    return math.isclose(this, that)

  def _basemixin_eq(this, that):
    return this == that

  def _coll_eq(this, that, collname='COLLECTION'):
    retval = SetEq(this)
    if len(this) != len(that):
      retval.differs.append(('len({})'.format(collname), len(this), len(that)))
      return retval
    
    for i, (item1, item2) in enumerate(zip(this, that)):
      for key, v1, v2 in (item1 == item2).differs:
        retval.differs.append(('{}[{}].{}'.format(collname, i, key), v1, v2))

    return retval

  def __eq__(self, other) -> Eq:
    if isinstance(self.obj, Collection):
      self.differs.extend(SetEq._coll_eq(self.obj, other.obj).differs)
      return self

    for key in self.clz.eq_keys:
      v1 = self.obj.__getattribute__(key) 
      v2 = other.obj.__getattribute__(key)
      assert type(v1) == type(v2), '{} vs {} for {}'.format(type(v1), type(v2), key)

      with_prefix = lambda x : key + '.' + x 

      if isinstance(v1, Collection):
        self.differs.extend(SetEq._coll_eq(v1, v2, key).differs)

      elif isinstance(v1, BaseMixin):
        for k, v1_, v2_ in SetEq._basemixin_eq(v1, v2).differs:
          self.differs.append((with_prefix(k), v1_, v2_))

      elif type(v1) is float:
        if SetEq._float_eq(v1, v2) is False:
          self.differs.append((key, v1, v2))
      
      else:
        if v1 != v2:
          self.differs.append((key, v1, v2))

    return self

  def __bool__(self):
    return len(self.differs) == 0

  def __str__(self):
    return 'SetEq{' + ';'.join(['({}, {}, {})'.format(*t) for t in self.differs]) + '}'

  def __len__(self):
    return self.differs.__len__()
  
  def difference(self, other):
    # tuples are weird
    differs = []
    for k, v1, v2 in self.differs:
      # try to find k in other.differs
      found = False
      for k_, v1_, v2_ in other.differs:
        if k == k_ and v1 == v1_ and v2 == v2_: 
          found = True
          break
      if not found: differs.append((k, v1, v2))
    return differs

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

  immutable_fields = ['intervention']
  coersions = {}

  def __init__(self, intervention):
    self._in_init = True
    self.intervention = intervention
    #self.schema = intervention.toybox.schema_for_state()


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
    
    # Need to force monotonicity of _in_init
    if name == '_in_init' and value is True and name in existing_attrs:
      raise MutationError(name)
    super().__setattr__(name, self.coersions[name](value) if name in self.coersions else value)

    # Only okay to add fields during initialization.
    if self._in_init: return
    if self.intervention is None: raise InterventionNoneError()
    assert isinstance(self.intervention, Intervention), '{}\t{}'.format(type(self.intervention), self.intervention)

    if name in self.immutable_fields: # and not :
      raise MutationError(name)
    if adding_new:
      raise MutationError("Cannot add new field %s to %s" % (name, self.__class__.__name__))
    if name != '_in_init':
      # Don't want to set dirty_state when we are flipping init
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

  #@abstractmethod
  def make_models(modelmod, data, game, objclassname, *attributes): 
    outdir = modelmod.replace('.', os.sep) #+ os.sep + objclassname.lower()
    os.makedirs(outdir, exist_ok=True)
    with open('resources/basemixin_template.py', 'r') as inf:
      with open(outdir + os.sep + '__init__.py', 'w') as outf:
        outf.write(inf.read().format(
          attribute_names= ', '.join(attributes), 
          game=game, 
          obj=objclassname))
      


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

  def make_models(modelmod, data, 
    game_name=None, 
    collmod_name=None, 
    coll_name=None, 
    coll_class=None, 
    elt_name=None):
    
    assert game_name,    'Argument {} is required for make_models on a collection object'.format(game_name)
    assert collmod_name, 'Argument {} is required for make_models on a collection object'.format(collmod_name)
    assert coll_name,    'Argument {} is required for make_models on a collection object'.format(coll_name)
    assert coll_class,   'Argument {} is required for make_models on a collection object'.format(coll_class)
    assert elt_name,     'Argument {} is required for make_models on a collection object'.format(elt_name)

    outdir = modelmod.replace('.', os.sep) + os.sep + coll_name
    os.makedirs(outdir, exist_ok=True)
    with open(outdir + os.sep + '__init__.py', 'w') as fout:
      with open('resources/collection_template.py', 'r') as fin:
        fout.write(fin.read().format(
          game_name=game_name, 
          collmod_name=collmod_name,
          coll_name=coll_name,
          coll_class=coll_class, 
          elt_name=elt_name))


        
class Intervention(ABC):

  def __init__(self, tb: Toybox, game_name: str, clz: type, modelmod=None, data=None, eq_mode=StandardEq):
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
    self.eq_mode = eq_mode

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
    self.clz.make_models(self.modelmod, self.data)

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


