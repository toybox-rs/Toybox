from toybox.toybox import LEFT, RIGHT, NOOP, UP, DOWN, BUTTON1

NOOP_STR = NOOP 
FIRE_STR = "FIRE"
UP_STR = UP
RIGHT_STR = RIGHT
LEFT_STR = LEFT
DOWN_STR = DOWN
UPFIRE_STR = "UPFIRE"
RIGHTFIRE_STR = "RIGHTFIRE"
LEFTFIRE_STR = "LEFTFIRE"
DOWNFIRE_STR = "DOWNFIRE"
BUTTON1_STR = BUTTON1

# Copied from, and required by, baselines
ACTION_MEANING = {
    0 : "NOOP",
    1 : "FIRE",
    2 : "UP",
    3 : "RIGHT",
    4 : "LEFT",
    5 : "DOWN",
    6 : "UPRIGHT",
    7 : "UPLEFT",
    8 : "DOWNRIGHT",
    9 : "DOWNLEFT",
    10 : "UPFIRE",
    11 : "RIGHTFIRE",
    12 : "LEFTFIRE",
    13 : "DOWNFIRE",
    14 : "UPRIGHTFIRE",
    15 : "UPLEFTFIRE",
    16 : "DOWNRIGHTFIRE",
    17 : "DOWNLEFTFIRE",
}

ACTION_LOOKUP = { v : k for (k, v) in ACTION_MEANING.items() }