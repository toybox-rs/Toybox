import importlib
import os
from toybox.interventions.{game_name} import {coll_class}

def sample(*args, **kwargs):
  coll = []
  intervention = kwargs['intervention'] 
  for elti in sorted(os.listdir(os.path.dirname(__file__))):
    if elti.startswith('{elt_name}'):
      mod = importlib.import_module('{collmod_name}.' + elti)
      coll.append(mod.sample(*args, **kwargs).encode())
  return {coll_class}.decode(intervention, coll, {coll_class})