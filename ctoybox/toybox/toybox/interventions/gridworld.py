from toybox.interventions.base import *
import json
"""An API for interventions on GridWorld."""

class GridWorldIntervention(Intervention):

    def __init__(self, tb, game_name='gridworld'):
        # check that the simulation in tb matches the game name.
        Intervention.__init__(self, tb, game_name)

    def set_world(self, fname): 
        with open(fname) as f:
            data = json.load(f)
            for k in data.keys(): 
                self.config[k] = data[k]


if __name__ == "__main__":
    with Toybox('gridworld') as tb:
        state = tb.to_state_json()
        config = tb.config_to_json()

        print(config)
        
        # save a sample starting state and config
        with open('toybox/toybox/interventions/defaults/gridworld_state_default.json', 'w') as outfile:
            json.dump(state, outfile)

        with open('toybox/toybox/interventions/defaults/gridworld_config_default.json', 'w') as outfile:
            json.dump(config, outfile)


        with GridWorldIntervention(tb) as intervention:
            gridfile = "../tb_gridworld/sample_grids/wall_unreachable.txt"
            intervention.set_world(gridfile)

        print(tb.config_to_json())
