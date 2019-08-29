from toybox.interventions.base import *
import json
"""An API for interventions on GridWorld."""

class GridWorldIntervention(Intervention):

    def __init__(self, tb, game_name='gridworld'):
        # check that the simulation in tb matches the game name.
        Intervention.__init__(self, tb, game_name)



if __name__ == "__main__":
  import argparse 
  parser = argparse.ArgumentParser(description='test Amidar interventions')
  parser.add_argument('--partial_config', type=str, default="null")
  parser.add_argument('--save_json', type=bool, default=False)

  args = parser.parse_args()

  with Toybox('gridworld') as tb:
    state = tb.to_state_json()
    config = tb.config_to_json()

    print(config)

    if args.save_json:
        # save a sample starting state and config
        with open('toybox/interventions/defaults/gridworld_state_default.json', 'w') as outfile:
            json.dump(state, outfile)

        with open('toybox/interventions/defaults/gridworld_config_default.json', 'w') as outfile:
            json.dump(config, outfile)

    with GridWorldIntervention(tb) as intervention:
        gridfile = "../tb_gridworld/sample_grids/wall_unreachable.txt"
        intervention.set_partial_config(gridfile)

    print(tb.config_to_json())
