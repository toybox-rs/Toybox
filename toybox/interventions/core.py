from toybox.interventions.base import * 
import typing
import math
import random
from pymonad.Either import *

# Result
Right.__and__ = lambda a, b: b

# Error
Left.__and__ = lambda a, b: a 


def eq(v1, v2, name='unknown'):
  assert type(v1) == type(v2)
  eqfn = math.isclose if type(v1) == float or type(v2) == float else (lambda x, y: x == y)
  if eqfn(v1, v2):
      return Result(True)
  e = Error('error in {}: {} not equal to {}'.format(name, v1, v2))
  e.name = name
  return e

def eq_map(fieldmap):
  names = list(fieldmap.keys())
  random.shuffle(names)
  retval = Result(True)

  for name in names:
    v1, v2 = fieldmap[name]
    retval = eq(v1, v2, name=name)
    if isinstance(retval, Error): return retval
  return retval


class Game(BaseMixin):
  """Base class for games. Supertype that contains common elements."""

  expected_keys = ['score', 'lives', 'rand', 'level']
  immutable_fields = []

  def __init__(self, intervention, score, lives, rand, level, *args, **kwargs):
    super().__init__(intervention)
    self.score = score
    self.rand = rand
    self.lives = lives
    self.level = level
    self.intervention = intervention
    # NO RESET OF _IN_INIT HERE
    # Game is an abstract class and should never be terminal
    # Python doesn't do great with multiple inheritence, which is 
    # what a truly abstract version of this class would look like.


class Direction(BaseMixin):

  expected_keys = []
  immutable_fields = []

  Up    = 'Up'
  Down  = 'Down'
  Left  = 'Left'
  Right = 'Right'

  directions = [Up, Down, Left, Right]

  def __init__(self, intervention, direction):
    super().__init__(intervention)
    assert direction in Direction.directions, '%s not found in directions' % direction
    self.direction = direction
    self._in_init = False

  def decode(intervention, direction, clz):
    return Direction(intervention, direction)

  def encode(self):
    return self.direction

  def __str__(self):
    return self.direction

  def __eq__(self, other):
    return eq(self.direction, other.direction, 'direction')


class Vec2D(BaseMixin):

  expected_keys = ['y', 'x']
  immutable_fields = []

  def __init__(self, intervention, x, y):
    super().__init__(intervention)
    self.x = x
    self.y = y
    self._in_init = False

  def __str__(self):
    return '({}, {})'.format(self.x, self.y)

  def __eq__(self, other):
    names = {
      'x': (self.x, other.x),
      'y': (self.y, other.y)
    }
    return eq_map(names)

class Color(BaseMixin):

  expected_keys = ['r', 'g', 'b', 'a']
  immutable_fields = []
  
  def __init__(self, intervention, r, g, b, a):
    super().__init__(intervention)
    self.r = r
    self.g = g 
    self.b = b 
    self.a = a   
    self._in_init = False

  def __str__(self):
    return "({}, {}, {}, {})".format(self.r, self.g, self.b, self.a)

  def __eq__(self, other):
    names = {
      'r' : (self.r, other.r), 
      'g' : (self.g, other.g),
      'b' : (self.b, other.b), 
      'a' : (self.a, other.a)
    }
    return eq_map(names)


class Collection(BaseMixin):

  expected_keys = []
  immutable_fields = ['intervention']

  def __init__(self, intervention, coll, elt_clz):
    super().__init__(intervention)
    self.elt_clz = elt_clz
    self.coll = [elt_clz.decode(intervention, elt, elt_clz) for elt in coll]
    # SAME DEAL AS GAME - THIS SHOULD ALWAYS BE ABSTRACT, HENCE NO RESET OF IN_INIT

  def __eq__(self, other):
      retval = Result(None)
      for i in range(len(self)):
          retval = eq(self[i], other[i], '{}[{}]'.format(self.elt_clz, i))
          if isinstance(retval, Error):
              return retval
      return retval


  def __str__(self):
    return '[{}]'.format(', '.join([str(c) for c in self.coll]))
      
  def __iter__(self): return self.coll.__iter__()

  def __getitem__(self, key): return self.coll.__getitem__(key)

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


class SpriteData(BaseMixin):
  
  expected_keys = ['x', 'y', 'data']
  immutable_fields = ['intervention', 'data']

  def __init__(self, intervention, x=None, y=None, data=None):
    super().__init__(intervention)
    self.x = x
    self.y = y
    self.data = ColorCollectionCollection.decode(intervention, data, None)
    self._in_init = False

  def __eq__(self, other):
    names = {
      'x': eq(self.x, other.x),
      'y': eq(self.y, other.y),
      'data': eq(self.data, other.data)
    } 
    return eq_map(names)

  def __str__(self):
    return 'Sprite at {}, {}'.format(self.x, self.y)


class ColorCollectionCollection(BaseMixin):

  expected_keys = []
  immutable_fields = []

  def __init__(self, intervention, sprites):
    super().__init__(intervention)
    self.coll = []
    for coll in sprites:
      self.coll.append([Color.decode(intervention, datum, Color) for datum in coll])
    self._in_init = False

  def __eq__(self, other):
    result = Result(None)
    for i in range(len(self.coll)):
      row = self[i]
      for j in range(len(row)):
        result = eq(row[j], other[i][j], '{}[{}][{}]'.format(ColorCollectionCollection.__name__, i, j))
        if isinstance(result, Error):
          return result
    return result

  def decode(intervention, coll, clz):
    return ColorCollectionCollection(intervention, coll)

  def encode(self):
    retval = []
    for colors in self.coll:
      retval.append([c.encode() for c in colors])
    return retval