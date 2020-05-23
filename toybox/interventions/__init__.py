from .breakout import BreakoutIntervention, Breakout
from .amidar import AmidarIntervention, Amidar
from .space_invaders import SpaceInvadersIntervention, SpaceInvaders
from .core import Game

def get_intervener(game_name):
  return {
    'breakout'     : BreakoutIntervention,
    'amidar'       : AmidarIntervention,
    'spaceinvaders': SpaceInvadersIntervention
  }[game_name]

def get_state_object(game_name):
  return {
    'breakout'     : Breakout, 
    'amidar'       : Amidar,
    'spaceinvaders': SpaceInvaders
  }[game_name]