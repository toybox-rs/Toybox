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
    assert direction in Direction.directions
    self.direction = direction

  def decode(intervention, direction, clz):
    return Direction(intervention, direction)

  def encode(self):
    return self.direction


