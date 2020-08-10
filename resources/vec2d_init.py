from . import x, y
from toybox.interventions.core import Vec2D

def sample(*args, **kwargs):
  intervention = kwargs['intervention']
  obj = { 'x' : x.sample(*args, **kwargs), 'y' : y.sample(*args, **kwargs)}
  return Vec2D.decode(intervention, obj, Vec2D)