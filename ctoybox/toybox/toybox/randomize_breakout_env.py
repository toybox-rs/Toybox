import random
import math

import toybox
#from toybox import Toybox
from toybox.toybox import Toybox
#from toybox.envs.atari.base import ToyboxBaseEnv
#from toybox.envs.atari.amidar import AmidarEnv
#from toybox.envs.atari.breakout import BreakoutEnv


def get_starting_state_json(): 
	# start a toybox instance 
	tb = Toybox('breakout')

	# export JSON
	breakout_json = tb.to_json()
	return breakout_json, tb


def select_ball_state(tb, state_json): 
	# get window size
	game_size = (tb.get_height(), tb.get_width())

	# set x, y location
	state_json["ball"]["position"]["x"] = int(random.random() * game_size[1])
	state_json["ball"]["position"]["y"] = int(random.random() * game_size[0])

	# set angle
	direction = random.random() * 360
	# convert to radians, polar

	# set velocity
	# set accelleration

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
	# random configuration of bricks
	for brick in range(len(state_json["bricks"])):
		state_json["bricks"][brick]["alive"] = True if random.random() > p else False
	return state_json


def select_paddle_state(tb, state_json): 
	# paddle position

	# paddle size

	return state_json 


def set_score(tb, state_json): 
	game_score = 0
	for brick in range(len(state_json["bricks"])):
		if state_json["bricks"][brick]["alive"]: 
			game_score += state_json["bricks"][brick]["points"]

	state_json["points"] = game_score
	return state_json


def generate_state(): 
	breakout_json, tb = get_starting_state_json()
	breakout_json = select_ball_state(tb, breakout_json)
	breakout_json = select_num_lives(tb, breakout_json)
	breakout_json = set_bricks(tb, breakout_json)
	breakout_json = set_score(tb, breakout_json)
	breakout_json = select_paddle_state(tb, breakout_json)

	return breakout_json, tb


if __name__ == "__main__": 
	breakout_json, tb = generate_state()
	tb.write_json(breakout_json)
	tb.save_frame_image("test_random_gen_breakout.png")



