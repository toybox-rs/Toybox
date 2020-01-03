from toybox.interventions.base import *
from toybox.interventions.core import *
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


class Player(BaseMixin):

    expected_keys = ['x', 'y', 'w', 'h', 'speed', 'color', 'alive', 'death_counter', 'death_hit_1']
    immutable_fields = []

    def __init__(self, intervention, 
        x=None, y=None, w=None, h=None, speed=None, color=None, 
        alive=None, death_counter=None, death_hit_1=None):

        self.intervention = intervention
        self.x = x
        self.y = y
        self.w = w 
        self.h = h 
        self.speed = speed
        self.color = Color.decode(intervention, color, Color)
        self.alive = alive
        self.death_counter = death_counter
        self.death_hit_1 = death_hit_1


class Laser(BaseMixin):

    expected_keys = ['y', 'x', 'w', 'h', 't', 'movement', 'speed', 'color']
    immutable_fields = []

    def __init__(self, intervention, 
        x=None, y=None, w=None, h=None, speed=None, color=None, 
        t=None, movement=None):

        self.intervention = intervention
        self.x = x
        self.y = y
        self.w = w 
        self.h = h
        self.t = t 
        self.movement = Direction.decode(intervention, movement, Direction)
        self.speed = speed
        self.color = Color.decode(intervention, color, Color)


class LaserCollection(Collection):

    expected_keys =[] 
    immutable_fields = ['intervention']

    def __init__(self, intervention, lasers):
        super().__init__(intervention, lasers, Laser)

class SpriteDataCollection(Collection):

    expected_keys = []
    
    def __init__(self, intervention, sprites):
        super().__init__(intervention, sprites, SpriteData)

    def decode(intervention, sprites, clz):
        return SpriteDataCollection(intervention, sprites)


class Ufo(BaseMixin):

    expected_keys = ['x', 'y', 'appearance_counter', 'death_counter']
    immutable_fields = []

    def __init__(self, intervention, x=None, y=None, appearance_counter=None, death_counter=None):

        self.intervention       = intervention 
        self.x                  = x
        self.y                  = y
        self.appearance_counter = appearance_counter
        self.death_counter      = death_counter


class Enemy(BaseMixin):

    expected_keys = ['x', 'y', 'row', 'col', 'id', 'alive', 'points', 'death_counter']
    immutable_fields = ['intervention']

    def __init__(self, intervention, x=None, y=None, row=None, col=None, id=None, alive=None, points=None, death_counter=None):

        self.intervention = intervention
        self.x = x
        self.y = y
        self.row = row
        self.col = col
        self.id = id
        self.alive = alive
        self.points = points
        self.death_counter = death_counter
    

class EnemyCollection(Collection):

    expected_keys = []
    immutable_fields = []

    def __init__(self, intervention, enemies):
        super().__init__(intervention, enemies, Enemy)


class EnemiesMovementState(BaseMixin):

    expected_keys = ['move_counter', 'move_dir', 'visual_orientation']
    immutable_fields = []

    def __init__(self, intervention, move_counter=None, move_dir=None, visual_orientation=None):

        self.intervention = intervention
        self.move_counter = move_counter
        self.move_dir = Direction.decode(intervention, move_dir, Direction)
        self.visual_orientation = visual_orientation


class SpaceInvadersIntervention(Intervention):

    def __init__(self, tb, game_name='space_invaders'):
        # check that the simulation in tb matches the game name.
        Intervention.__init__(self, tb, game_name, SpaceInvaders)

    # # getters
    # def get_jitter(self): 
    #     return self.config['jitter']

    # def lives_remaining(self): 
    #     return self.state['lives']




    # # atomic setters
    # def remove_mothership(self, banish_time): 
    #     self.state['ufo']['appearance_counter'] = banish_time


    # def set_player_speed(self, s): 
    #     self.state['ship']['speed'] = s


    # def set_player_location(self, pos): 
    #     assert check_position(pos, ['x', 'y'])
    #     self.state['ship']['x'] = pos['x']
    #     self.state['ship']['y'] = pos['y']


    # def set_player_size(self, dim): 
    #     assert check_position(dim, ['w', 'h'])
    #     self.state['ship']['w'] = pos['w']
    #     self.state['ship']['h'] = pos['h']


    # def set_shield_loc(self, sid, loc): 
    #     assert check_position(loc, ['x', 'y'])

    #     self.state['shields'][sid]['x'] = loc['x']
    #     self.state['shields'][sid]['y'] = loc['y']


    # # this may be a death animation thing
    # # mothership comes by on a period
    # def set_enemy_death_hits(self, eids, n): 
    #     for eid in eids: 
    #         self.state['enemies'][eid]['death_counter'] = n


    # def set_lives_remaining(self, n): 
    #     self.state['lives'] = n


    # def set_enemy_shot_delay(self, t):
    #     self.state['enemy_shot_delay'] = t 


    # def set_jitter(self, j): 
    #     self.config['jitter'] = j


    # def set_player_death_hit(self, n): 
    #     self.state['ship']['death_hit_1'] = False
    #     self.state['ship']['death_counter'] = n





    # # composite setters
    # def set_mothership(self, loc): 
    #     assert check_position(loc, ['x', 'y'])

    #     self.state['ufo']['appearance_counter'] = 0
    #     self.state['ufo']['x'] = loc['x']
    #     self.state['ufo']['y'] = loc['y']


    # def set_enemy_protocol();
    #     pass


    # # Define a custom intervention on the space invaders' targeting.
    # def customEnemyMovement():
    #     pass

    # def add_enemy(self, kwargs**): 
    #     assert check_position(kwargs, ['x', 'y'])

    #     e = {}
    #     e['id'] = len(self.state['enemies'])
    #     e['x'] = kwargs['x']
    #     e['y'] = kwargs['y']
    #     e['alive'] = True

    #     #e['col']
    #     #e['row']

    #     #e['move_down']
    #     #e['move_right']
    #     #e['orientation_init']

    #     # default to 20 points if 'points' is not defined in kwargs and enemy does not fit neatly in row/col
    #     row_score = self.config['row_scores'][e['row']] if e['row'] < len(self.config['row_scores']) else 20 
    #     e['points'] = kwargs['points'] if 'points' in kwargs.keys() else row_score

    #     #e['death_counter']
    #     #e['move_counter']

    #     self.state['enemies'].append(e)


    # def remove_enemy(self, eid): 
    #     eids = range(len(self.state['enemies']))
    #     eids.remove(eid)

    #     self.state['enemies'] = self.state['enemies'][eids]


    # def shift_enemies(self, shift_vector):
    #     assert check_position(shift_vector, ['x', 'y'])
    #     for e in self.state['enemies']: 
    #         e['x'] = e['x'] + shift_vector['x']
    #         e['y'] = e['y'] + shift_vector['y']


    # def add_shield(self, loc, shape_mat=None):
    #     assert check_position(loc, ['x', 'y'])

    #     new_s = {}
    #     new_s['x'] = loc['x']
    #     new_s['y'] = loc['y']

    #     # may need to insert some assertions about the structure of shape_mat
    #     if shape_mat is not None:
    #         new_s['data'] = shape_mat
    #     else: 
    #         # use default shield shape
            

    #     self.state['shields'].append(self.state['shields'])


    # def remove_shield(self, sid): 
    #     sids = range(len(self.state['shields']))
    #     sids = sids.remove(sid)

    #     self.state['shields'] = self.state['shields'][sids]


    # def set_shield_shape(self, sid, shape_mat):
    #     # may need to insert some assertions about the structure of shape_mat
    #     self.state['shields'][sid]['data'] = shape_mat

 

    # move player
    # shift enemies +- x, +- y
    # add/remove shields
    # alter shield shape
    # add/remove mothership 
    # change ship speed
    # make enemy/enemies take more than one hit
    # alter enemy_shot_delay
    # custom enemy firing pattern


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
