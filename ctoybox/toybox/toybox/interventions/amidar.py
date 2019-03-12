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
        return self.state['player']['position']

    def num_enemies(self):
        return len(self.state['enemies'])

    def jumps_remaining(self):
        return self.state['jumps']

    def regular_mode(self):
        return self.state['jump_timer'] == 0 and self.state['chase_timer'] == 0

    def jump_mode(self):
        return self.state['jump_timer'] > 0

    def chase_mode(self):
        self.state['chase_timer'] > 0

    def enemy_tiles(self):
        return [self.state['enemies'][i]['position'] for i in range(len(self.state['enemies']))]

    def enemy_caught(self, eid):
        return self.state['enemies'][eid]['caught']

    def any_enemy_caught(self, eid):
        return any([self.state['enemies'][i]['caught'] for i in range(len(self.state['enemies']))])

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
    import argparse 

    parser = argparse.ArgumentParser(description='test Amidar interventions')
    parser.add_argument('--partial_config', type=str, default="null")
    parser.add_argument('--save_json', type=bool, default=False)

    args = parser.parse_args()

    with Toybox('amidar') as tb:
        state = tb.to_state_json()
        config = tb.config_to_json()

    if args.save_json:
        # save a sample starting state and config
        with open('toybox/toybox/interventions/defaults/amidar_state_default.json', 'w') as outfile:
            json.dump(state, outfile)

        with open('toybox/toybox/interventions/defaults/amidar_config_default.json', 'w') as outfile:
            json.dump(config, outfile)