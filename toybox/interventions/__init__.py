from .breakout import BreakoutIntervention, Breakout
from .amidar import AmidarIntervention, Amidar
from .space_invaders import SpaceInvadersIntervention, SpaceInvaders
from .core import Game, StandardEq
from ctoybox import Toybox

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

def state_from_toybox(toybox: Toybox, eq_mode=StandardEq):
  state_obj = get_state_object(toybox.game_name)
  with get_intervener(toybox.game_name)(toybox) as intervention:
    return state_obj.decode(intervention, toybox.state_to_json(), state_obj)