from ctoybox import Input

NOOP_STR = Input._NOOP.upper() 
FIRE_STR = "FIRE"
UP_STR = Input._UP.upper()
RIGHT_STR = Input._RIGHT.upper()
LEFT_STR = Input._LEFT.upper()
DOWN_STR = Input._DOWN.upper()
UPFIRE_STR = "UPFIRE"
RIGHTFIRE_STR = "RIGHTFIRE"
LEFTFIRE_STR = "LEFTFIRE"
DOWNFIRE_STR = "DOWNFIRE"
BUTTON1_STR = Input._BUTTON1.upper()

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
