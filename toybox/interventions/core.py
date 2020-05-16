from toybox.interventions.base import * 
from typing import Any
import math
import random
import re
import sys
from sklearn.neighbors import KernelDensity
from numpy import array
try:
  import cPickle as pickle
except:
  import pickle


def distr(fname, data):
  # select bandwidth according to scotts rule
  # https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.gaussian_kde.html
  bandwidth = len(data)**(-1./5)
  kde = KernelDensity(bandwidth=bandwidth, kernel='epanechnikov')
  kde.fit(array(data).reshape(-1, 1))
  os.makedirs(os.path.dirname(fname), exist_ok=True)
  with open(fname + '.pck', 'wb') as f:
    pickle.dump(kde, f)
  with open(fname + '.py', 'w') as f:
    f.write("""try:
  import cPickle as pickle
except:
  import pickle

with open('{0}', 'rb') as f:
  kde = pickle.load(f)

def sample(*args, **kwargs):
  return float(kde.sample()[0][0])
      """.format(fname + '.pck'))


class Game(BaseMixin):
  """Base class for games. Supertype that contains common elements."""

  expected_keys = ['score', 'lives', 'rand', 'level']
  immutable_fields = BaseMixin.immutable_fields + ['rand', 'reset']
  coersions={
      'score' : lambda x : int(x),
      'lives' : lambda x : int(x),
      'level' : lambda x : int(x),
      'reset' : lambda x : bool(x)
  }

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

  def make_models(self, data):
    outdir = self.modelmod.replace('.', '/') + os.sep
    print('Creating models in {}'.format(outdir))

    distr(outdir + 'score', [d.score for d in data])
    distr(outdir + 'lives', [d.lives for d in data])
    distr(outdir + 'level', [d.level for d in data])


class Direction(BaseMixin):

  expected_keys = []
  immutable_fields = BaseMixin.immutable_fields

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


class Vec2D(BaseMixin):

  expected_keys = ['y', 'x']
  eq_keys = expected_keys
  immutable_fields = BaseMixin.immutable_fields

  def __init__(self, intervention, x, y):
    super().__init__(intervention)
    self.x = x
    self.y = y
    self._in_init = False

  def __str__(self):
    return '({}, {})'.format(self.x, self.y)

  def make_models(self, outdir, data):
    distr(outdir + os.sep + 'x', [d.x for d in data])
    distr(outdir + os.sep + 'y', [d.y for d in data])
    with open(outdir + os.sep + '__init__.py', 'w') as f:
      f.write("""from . import x, y
from toybox.interventions.core import Vec2D

def sample(*args, **kwargs):
  intervention = kwargs['intervention'] if 'intervention' in kwargs else None
  obj = { 'x' : x.sample(*args, **kwargs), 'y' : y.sample(*args, **kwargs)}
  return Vec2D.decode(intervention, obj, Vec2D)
""")
    
  
class Color(BaseMixin):

  expected_keys = ['r', 'g', 'b', 'a']
  eq_keys = expected_keys
  immutable_fields = BaseMixin.immutable_fields
  coersions = {
    'r': lambda x : max(0, min(255, int(x))),
    'g': lambda x : max(0, min(255, int(x))),
    'b': lambda x : max(0, min(255, int(x))),
    'a': lambda x : max(0, min(255, int(x)))
  }
  
  def __init__(self, intervention, r, g, b, a):
    super().__init__(intervention)
    self.r = r
    self.g = g 
    self.b = b 
    self.a = a   
    self._in_init = False

  def __str__(self):
    return "({}, {}, {}, {})".format(self.r, self.g, self.b, self.a)

  def make_models(self, outdir, data): 
    distr(outdir + os.sep + 'r', [d.r for d in data])
    distr(outdir + os.sep + 'g', [d.g for d in data])
    distr(outdir + os.sep + 'b', [d.b for d in data])
    distr(outdir + os.sep + 'a', [d.a for d in data])
    with open(outdir + os.sep + '__init__.py', 'w') as f:
      f.write("""from . import r, g, b, a
from toybox.interventions.core import Color

def sample(*args, **kwargs):
  intervention = kwargs['intervention'] if 'intervention' in kwargs else None
  obj = {
    'r' : r.sample(*args, **kwargs),
    'g' : g.sample(*args, **kwargs),
    'b' : b.sample(*args, **kwargs),
    'a' : a.sample(*args, **kwargs)
  }
  return Color.decode(intervention, obj, Color)
      """)



class SpriteData(BaseMixin):
  
  expected_keys = ['x', 'y', 'data']
  eq_keys = expected_keys
  immutable_fields = BaseMixin.immutable_fields + ['data']

  def __init__(self, intervention, x=None, y=None, data=None):
    super().__init__(intervention)
    self.x = x
    self.y = y
    self.data = ColorCollectionCollection.decode(intervention, data, None)
    self._in_init = False

  def __str__(self):
    return 'Sprite at {}, {}'.format(self.x, self.y)


class ColorCollectionCollection(BaseMixin):

  expected_keys = []
  immutable_fields = BaseMixin.immutable_fields + ['coll']

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


def get_property(s: Game, prop: str, setval=None) -> Any:
  """Gets or sets object property expressed as a string in the format
  that is returned by the generate_mutation_points function."""
  indexpat = re.compile('\[.*?\]')
  levels = prop.split('.')
  obj = s
  set_index = len(levels) - 1 # the index of the containing object of the property
  for level, prop in enumerate(levels):
    indices = indexpat.findall(prop)
    if indices:
      # get the index of the first index
      i = prop.index('[')
      # get the indexable object
      obj = obj.__getattribute__(prop[:i])
      for j, i in enumerate(indices): 
        obj = obj[int(i[1:-1])]
    else:
      if setval and level == set_index:
        obj.__setattr__(prop, setval)
      obj = obj.__getattribute__(prop)
      
  return obj
