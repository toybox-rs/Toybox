from toybox.interventions.base import *
import json
"""An API for interventions on Amidar."""

class AmidarIntervention(Intervention):

    def __init__(self, tb, game_name='amidar'):
        # check that the simulation in tb matches the game name.
        Intervention.__init__(self, tb, game_name)


    def num_tiles_unpainted(self):
        pass
        #return self.query_json('num_tiles_unpainted')
        
    def player_tile(self):
        pass
        #return self.query_json('player_tile')

    def num_enemies(self):
        pass
        #return self.query_json('num_enemies')

    def jumps_remaining(self):
        pass
        #return self.query_json('jumps_remaining')

    def regular_mode(self):
        pass
        #return self.query_json('regular_mode')

    def jump_mode(self):
        pass
        #return self.query_json('jump_mode')

    def chase_mode(self):
        pass
        #return self.query_json('chase_mode')

    def enemy_tiles(self):
        pass
        #return self.query_json('enemy_tiles')

    def enemy_caught(self, eid):
        pass
        #return self.query_json('enemy_caught', eid)

    def any_enemy_caught(self, eid):
        pass
        #num_enemies = self.amidar_num_enemies()
        #return any(self.amidar_enemy_caught(eid) for eid in range(num_enemies))

    # paint/unpaint tiles
    # paint/unpaint rectangles 
        # consider logic for tiles, rectangles both filled
    # move player to tile, (x,y)
    # move enemy(ies) to tile, (x,y)
    # begin jump mode
    # begin chase mode
    # begin regular mode
    # set, return enemy protocol 
    # get, set score
        # consider logic for score calculation
    # add/remove enemy
    # set number of jumps
    # set number of lives
    # random start state?


if __name__ == "__main__":
  with Toybox('amidar') as tb:
    state = tb.to_state_json()
    config = tb.config_to_json()

    # save a sample starting state and config
    with open('toybox/toybox/interventions/defaults/amidar_state_default.json', 'w') as outfile:
        json.dump(state, outfile)

    with open('toybox/toybox/interventions/defaults/amidar_config_default.json', 'w') as outfile:
        json.dump(config, outfile)