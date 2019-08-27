import random
import math
import numpy

import toybox
#from toybox import Toybox
from toybox import Toybox
from toybox.toybox.interventions import AmidarIntervention 
#from toybox.envs.atari.base import ToyboxBaseEnv
#from toybox.envs.atari.amidar import AmidarEnv
#from toybox.envs.atari.breakout import BreakoutEnv


def get_starting_state_json(): 
	# start a toybox instance 
	tb = Toybox('amidar')

	# export JSON
	amidar_json = tb.to_json()
	return amidar_json, tb


def select_num_lives(tb, state_json): 
	# set number of lives
	state_json["lives"] = math.ceil(random.random()*tb.rstate.lives())
	return state_json


def intialize_player(tb, state_json): 
    # grab random tile
    rand_tile = AmidarIntervention.get_random_tile_id()
    # check that the tile is walkable
    while not AmidarIntervention.check_is_tile(rand_tile): 
    	rand_tile = AmidarIntervention.get_random_tile_id()

    # get position for new tile
    pos = AmidarIntervention.tile_to_world(pos)
    # now set the position
    state_json['player']['position']['x'] = pos['x']
    state_json['player']['position']['y'] = pos['y']

    return state_json


# board completion 
def set_board_completion(tb, state_json, extreme):
	# calculate number of lives used
	with AmidarIntervention(tb) as intervention: 
		max_lives = intervention.start_lives()
	lives_used = max_lives - state_json["lives"]
	for life in lives_used: 
		tiles_to_remove = numpy.random.poisson() # ideally this is the average tiles painted per episode of some trained agent or human expert
		
		if extreme: 
			# randomly assign tiles on/off
			while tiles_to_remove > 0:  
				# randomly select unpainted tile and paint
				tile_id = AmidarIntervention.get_random_tile_id()
				state_json["board"][tile_id['tx']][tile_id['ty']] = "Painted"
				tiles_to_remove = tiles_to_remove - 1

		else: 
			# mimic board traversal 
			# for each life, select a path traveled by some previously trained agent 
				# threat to validity: we are introducing bias from agents trained on default Amidar envs
			# from start state, travel along path of length tiles_to_remove
			path_tiles = []
			while tiles_to_remove > 0: 
				# random path of length tiles_to_remove	
				# choose direction and move until junction
				pass 	
			for tile in path_tiles: 
				state_json["board"][tile['tx']][tile['ty']] = "Painted"

	# assign rectangle fills accordingly 


	return state_json


def set_score(tb, state_json): 
	# collect score from painted tiles, rectangles
	game_score = 0
	
	# collect score from painted tiles
	for row in range(len(state_json["board"]["tiles"])):
		for tile in range(length(state_json["board"]["tiles"][row])): 
			if state_json["board"]["tiles"][row][tile] == "Painted": 
				# vertical segments give you 1, horizontal give you length
				if row in [0, 6, 12, 18, 24, 30]: 
					game_score = game_score + 1
				else:
					# calculate length of tile 
					tile_length = 1
					game_score = game_score + tile_length  

	state_json["points"] = game_score
	return state_json


def select_enemy_states(tb, state_json, extreme): 
		# randomize tile of enemy

	for enemy in range(AmidarIntervention.num_enemies(tb)):
		if extreme: 
			# select random tile for each enemy from lookup protocol positions; use AmidarMvmt protocol

		else: 
			# select random tile; keep configuration of enemies locked



def generate_state(extreme=False, n_jumps=0): 
	amidar_json, tb = get_starting_state_json()
	if extreme:
		amidar_json = intialize_player(tb, amidar_json)
	amidar_json = select_num_lives(tb, amidar_json)
	amidar_json = set_board_completion(tb, amidar_json, extreme)
	amidar_json = set_score(tb, amidar_json)
	amidar_json = select_enemy_states(tb, amidar_json, extreme)

	amidar_json["jumps"] = n_jumps

	return amidar_json, tb


if __name__ == "__main__": 
	amidar_json, tb = generate_state()
	tb.write_json(amidar_json)
	tb.save_frame_image("test_random_gen_amidar.png")



