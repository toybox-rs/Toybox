import random
import math

import toybox
from abc import ABC
from toybox.interventions import *


def get_starting_state_json(): 
	# start a toybox instance 
	tb = toybox.toybox.Toybox('breakout')

	# export JSON
	breakout_json = tb.to_state_json()
	return breakout_json, tb


def select_ball_state(tb, state_json, extreme=False): 
	# get window size
	# currently hard-coded to default
	game_size = (123, 216)
	if extreme: 
		# all Breakout board available
		# set x, y location
		state_json["balls"][0]["position"]["x"] = int(random.random() * game_size[1]) + 12
		state_json["balls"][0]["position"]["y"] = int(random.random() * game_size[0]) + 25
	else: 
		# only space under the bricks available
		state_json["balls"][0]["position"]["x"] = int(random.random() * game_size[1]) + 12
		state_json["balls"][0]["position"]["y"] = int(random.random() * 40) + 80


	# set x, y location
	print(state_json["balls"][0]["position"])

	# use default velocity, acceleration settings
	#speed = tb.to_config_json()["ball_speed_slow"]
	speed = 2.0
	# select random ball angle
	angle = math.radians(random.random() * 360)

	# set angle
	state_json["balls"][0]["velocity"]["y"] = speed * math.sin(math.radians(angle))
	state_json["balls"][0]["velocity"]["x"] = speed * math.cos(math.radians(angle))

	return state_json


def select_num_lives(tb, state_json): 
	# set number of lives
	state_json["lives"] = math.ceil(random.random()*tb.rstate.lives())
	return state_json


def set_default_concepts(tb, state_json): 
	_, defaults = get_starting_state_json()
	state_json["config"] = defaults["config"]

	# ball radius 
	# points? calculate from brick states
	# brick setup (6 rows, total points, num across)

	return state_json


def set_bricks(tb, state_json, p = 0.5): 
	# sample from partial ordering of bricks 
	
	# this function assumes state_json represents a state with a full set of bricks
	total_bricks = len(state_json["bricks"])

	# hard-coding defaults for now
	num_cols = 18
	num_rows = 6

	# choose a number of bricks to remove
	num_bricks_to_remove = random.random()*total_bricks*.5
	print(num_bricks_to_remove)

	# store IDs for reachable bricks - those in bottom row to start
	reachable_bricks = set([num_rows*i + num_rows-1 for i in range(num_cols)])

	while num_bricks_to_remove > 0 and not len(reachable_bricks) == 0:
		b = random.sample(reachable_bricks, 1)[0]
		reachable_bricks.remove(b)
		if state_json['bricks'][b]['alive']:
			# remove brick
			state_json['bricks'][b]['alive'] = False
			# decrement count to remove
			num_bricks_to_remove -= 1

			# update reachable bricks 
			# make sure to exclude cases where we removed a diagonal brick 
			# and subsequent removals are not reachable
			b_row = state_json["bricks"][b]["row"]
			b_col = state_json["bricks"][b]["col"]
			open_below = b_row == num_rows-1
			if not open_below: 
				below_brick = b + 1
				open_below = not state_json["bricks"][below_brick]["alive"]
				
			if open_below:
				for i in range(-1,1): 
					for j in range(-1,2): 
						if not (i == j and i == 0): 
							row = b_row + i 
							col = b_col + j 

							if row >= 0 and row < num_rows and col >= 0 and col < num_cols: 
								b_id = num_rows*col + row
								reachable_bricks.add(b_id)

	return state_json


def select_paddle_state(tb, state_json, extreme): 
	# paddle position
	if extreme: 
		# sample from full x axis
		state_json["paddle"]["position"]["x"] = 120 + int(random.random()*100 - 5) 
	else:
		# add +- 5 pixels
		state_json["paddle"]["position"]["x"] = 120 + int(random.random()*10 - 5) 
	return state_json 


def set_score(tb, state_json): 
	game_score = 0
	for brick in range(len(state_json["bricks"])):
		if state_json["bricks"][brick]["alive"]: 
			game_score += state_json["bricks"][brick]["points"]

	state_json["points"] = game_score
	return state_json


def generate_state(extreme=False): 
	breakout_json, tb = get_starting_state_json()

	breakout_json = select_ball_state(tb, breakout_json, extreme)
	breakout_json = select_num_lives(tb, breakout_json)
	breakout_json = set_bricks(tb, breakout_json, 0.5)
	breakout_json = set_score(tb, breakout_json)
	breakout_json = select_paddle_state(tb, breakout_json, extreme)

	return breakout_json, tb


if __name__ == "__main__":
	print("random breakout state") 

	breakout_json, tb = generate_state()
	tb.write_state_json(breakout_json)
	tb.save_frame_image("test_random_gen_breakout.png")



