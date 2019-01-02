import random
import math

from toybox.toybox import Toybox
from toybox.randomize_breakout_env import *

breakout_json, tb = generate_state()
tb.write_json(breakout_json)
tb.save_frame_image("test_random_gen_breakout.png")