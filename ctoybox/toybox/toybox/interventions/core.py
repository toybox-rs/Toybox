from toybox.interventions.base import * 

class Game(BaseMixin):
  """Base class for games. Supertype that contains common elements."""

  expected_keys = ['score', 'lives', 'rand', 'level']
  immutable_fields = []

  def __init__(self, intervention, score, lives, rand, level, *args, **kwargs):
    self.score = score
    self.rand = rand
    self.lives = lives
    self.level = level
    self.intervention = intervention

class Direction(BaseMixin):

  expected_keys = []
  immutable_fields = []

  Up    = 'Up'
  Down  = 'Down'
  Left  = 'Left'
  Right = 'Right'

  directions = [Up, Down, Left, Right]

  def __init__(self, intervention, direction):
    self.intervention = intervention
    assert direction in Direction.directions, '%s not found in directions' % direction
    self.direction = direction

  def decode(intervention, direction, clz):
    return Direction(intervention, direction)

  def encode(self):
    return self.direction


class Vec2D(BaseMixin):

  expected_keys = ['y', 'x']
  immutable_fields = []

  def __init__(self, intervention, x, y):
    self.intervention = intervention
    self.x = x
    self.y = y

class Color(BaseMixin):

  expected_keys = ['r', 'g', 'b', 'a']
  immutable_fields = []
  
  def __init__(self, intervention, r, g, b, a):
    self.intervention = intervention
    self.r = r
    self.g = g 
    self.b = b 
    self.a = a   


class Collection(BaseMixin):

  expected_keys = []
  immutable_fields = ['intervention']

  def __init__(self, intervention, coll, elt_clz):
    self.intervention = intervention
    self.elt_clz = elt_clz
    self.coll = [elt_clz.decode(intervention, elt, elt_clz) for elt in coll]

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
    self.intervention = intervention
    self.x = x
    self.y = y
    self.data = ColorCollectionCollection.decode(intervention, data, None)


class ColorCollectionCollection(BaseMixin):

  expected_keys = []
  immutable_fields = []

  def __init__(self, intervention, sprites):
    self.intervention = intervention
    self.coll = []
    for coll in sprites:
      self.coll.append([Color.decode(intervention, datum, Color) for datum in coll])

  def decode(intervention, coll, clz):
    return ColorCollectionCollection(intervention, coll)

  def encode(self):
    retval = []
    for colors in self.coll:
      retval.append([c.encode() for c in colors])
    return retval