from . import {attribute_names}
from toybox.interventions.{game} import {obj}
from toybox.interventions.base import BaseMixin

def sample(*args, **kwargs):
  intervention = kwargs['intervention']
  obj = {{}}
  for attr in [{attribute_names}]:
    val = attr.sample(*args, **kwargs)
    obj[attr.__name__.split('.')[-1]] = val.encode() if isinstance(val, BaseMixin) else val
  return {obj}.decode(intervention, obj, {obj})