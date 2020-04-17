from toybox.interventions.base import *
from toybox.interventions.core import *
try:
    import ujson as json
except:
    import json
"""An API for interventions on Space Invaders."""

class SpaceInvaders(Game):

    expected_keys = Game.expected_keys + ['enemy_lasers', 'ufo', 'rand', 'shields', 'enemies_movement', 'life_display_timer', 'ship', 'lives', 'score', 'ship_laser', 'enemy_shot_delay', 'level', 'enemies']
    immutable_fields = Game.immutable_fields

    def __init__(self, intervention,
        score=None, ship_laser=None, enemies=None, rand=None, 
        ufo=None, ship=None, life_display_timer=None, shields=None, 
        enemies_movement=None, lives=None, level=None, enemy_lasers=None, enemy_shot_delay=None):

        super().__init__(intervention, score, lives, rand, level)
        self.ship               =               Player.decode(intervention, ship,             Player)
        self.ship_laser         =                Laser.decode(intervention, ship_laser,       Laser) if ship_laser else None
        self.shields            = SpriteDataCollection.decode(intervention, shields,          SpriteDataCollection)
        self.enemies            =      EnemyCollection.decode(intervention, enemies,          EnemyCollection)
        self.enemies_movement  = EnemiesMovementState.decode(intervention, enemies_movement, EnemiesMovementState)
        self.enemy_lasers       =      LaserCollection.decode(intervention, enemy_lasers,     LaserCollection)
        self.ufo                =                  Ufo.decode(intervention, ufo,              Ufo)

        self.life_display_timer = life_display_timer
        self.enemy_shot_delay   = enemy_shot_delay
        self._in_init           = False


class Player(BaseMixin):

    expected_keys = ['x', 'y', 'w', 'h', 'speed', 'color', 'alive', 'death_counter', 'death_hit_1']
    immutable_fields = []

    def __init__(self, intervention, 
        x=None, y=None, w=None, h=None, speed=None, color=None, 
        alive=None, death_counter=None, death_hit_1=None):

        super().__init__(intervention)
        self.x = x
        self.y = y
        self.w = w 
        self.h = h 
        self.speed = speed
        self.color = Color.decode(intervention, color, Color)
        self.alive = alive
        self.death_counter = death_counter
        self.death_hit_1 = death_hit_1
        self._in_init = False

class Laser(BaseMixin):

    expected_keys = ['y', 'x', 'w', 'h', 't', 'movement', 'speed', 'color']
    immutable_fields = []

    def __init__(self, intervention, 
        x=None, y=None, w=None, h=None, speed=None, color=None, 
        t=None, movement=None):

        super().__init__(intervention)
        self.x = x
        self.y = y
        self.w = w 
        self.h = h
        self.t = t 
        self.movement = Direction.decode(intervention, movement, Direction)
        self.speed = speed
        self.color = Color.decode(intervention, color, Color)
        self._in_init = False

class LaserCollection(Collection):

    expected_keys =[] 
    immutable_fields = ['intervention']

    def __init__(self, intervention, lasers):
        super().__init__(intervention, lasers, Laser)
        self._in_init = False

class SpriteDataCollection(Collection):

    expected_keys = []
    
    def __init__(self, intervention, sprites):
        super().__init__(intervention, sprites, SpriteData)
        self._in_init = False

    def decode(intervention, sprites, clz):
        return SpriteDataCollection(intervention, sprites)


class Ufo(BaseMixin):

    expected_keys = ['x', 'y', 'appearance_counter', 'death_counter']
    immutable_fields = []

    def __init__(self, intervention, x=None, y=None, appearance_counter=None, death_counter=None):

        super().__init__(intervention)
        self.x                  = x
        self.y                  = y
        self.appearance_counter = appearance_counter
        self.death_counter      = death_counter
        self._in_init = False

class Enemy(BaseMixin):

    expected_keys = ['x', 'y', 'row', 'col', 'id', 'alive', 'points', 'death_counter']
    immutable_fields = ['intervention']

    def __init__(self, intervention, x=None, y=None, row=None, col=None, id=None, alive=None, points=None, death_counter=None):

        super().__init__(intervention)
        self.x = x
        self.y = y
        self.row = row
        self.col = col
        self.id = id
        self.alive = alive
        self.points = points
        self.death_counter = death_counter
        self._in_init = False
    

class EnemyCollection(Collection):

    expected_keys = []
    immutable_fields = []

    def __init__(self, intervention, enemies):
        super().__init__(intervention, enemies, Enemy)
        self._in_init = False


class EnemiesMovementState(BaseMixin):

    expected_keys = ['move_counter', 'move_dir', 'visual_orientation']
    immutable_fields = []

    def __init__(self, intervention, move_counter=None, move_dir=None, visual_orientation=None):

        super().__init__(intervention)
        self.move_counter = move_counter
        self.move_dir = Direction.decode(intervention, move_dir, Direction)
        self.visual_orientation = visual_orientation
        self._in_init = False


class SpaceInvadersIntervention(Intervention):

    def __init__(self, tb, game_name='space_invaders'):
        # check that the simulation in tb matches the game name.
        Intervention.__init__(self, tb, game_name, SpaceInvaders)

    def get_jitter(self): 
        return self.config['jitter']

    def set_jitter(self, p):
        self.dirty_config = True
        self.config['jitter'] = p

    def remove_mothership(self, banish_time): 
        self.game.ufo.appearance_counter = -1

    def get_player(self):
        return self.game.ship


if __name__ == "__main__":
  import argparse 
  parser = argparse.ArgumentParser(description='test Space Invaders interventions')
  parser.add_argument('--partial_config', type=str, default="null")
  parser.add_argument('--save_json', type=bool, default=False)

  args = parser.parse_args()

  with Toybox('space_invaders') as tb:
    state = tb.to_state_json()
    config = tb.config_to_json()

    if args.save_json:
      # save a sample starting state and config
      with open('toybox/toybox/interventions/defaults/space_invaders_state_default.json', 'w') as outfile:
          json.dump(state, outfile)

      with open('toybox/toybox/interventions/defaults/space_invaders_config_default.json', 'w') as outfile:
          json.dump(config, outfile)

    with SpaceInvadersIntervention(tb) as intervention:
        intervention.game.lives = 1 
        assert intervention.dirty_state
