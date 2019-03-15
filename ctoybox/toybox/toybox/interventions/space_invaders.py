from toybox.interventions.base import *
import json
"""An API for interventions on Space Invaders."""

class SpaceInvadersIntervention(Intervention):

    def __init__(self, tb, game_name='space_invaders'):
        # check that the simulation in tb matches the game name.
        Intervention.__init__(self, tb, game_name)

    # Define a custom intervention on the space invaders' targeting.
    def customEnemyMovement():
        pass


    # move player
    # shift enemies +- x, +- y
    # add/remove shields
    # add/remove mothership 
    # change ship speed
    # make enemy/enemies take more than one hit
    # get/set lives
    # alter enemy_shot_delay
    # custon enemy firing pattern


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