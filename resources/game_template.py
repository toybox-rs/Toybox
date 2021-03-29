# THIS SHOULD NOT BE EXECUTABLE. 
# This file should only be used for generating other 
# executables. Using the .py extension for syntax highlighting.

from ctoybox import Toybox
from toybox.interventions.{game} import {intervention}
from toybox.interventions.core import get_property, Collection
from . import * 
import importlib

def sample(*args, **kwargs):
  with Toybox({game}) as tb:
    with {intervention}(tb) as intervention:
      game = intervention.game
      for key, v in vars(game).items():
        if key in game.immutable_fields and not isinstance(v, Collection): continue

        mod = importlib.import_module(kwargs['modelmod'] + '.' + key)
        val = mod.sample(*args, **kwargs)
        if key in game.coersions: val = game.coersions[key](val)
        if __debug__: 
          before = get_property(game, key)

        if key in game.immutable_fields:
          v.clear()
          for item in val:
            v.append(item)
        else: 
          after = get_property(game, key, setval=val)
      return game