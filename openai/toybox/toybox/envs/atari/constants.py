from toybox.toybox import LEFT, RIGHT, NOOP, UP, DOWN

NOOP_STR = NOOP 
UP_STR = UP
RIGHT_STR = RIGHT
LEFT_STR = LEFT
DOWN_STR = DOWN


NOOP_ID = 0
UP_ID = 2
RIGHT_ID = 3
LEFT_ID = 4
DOWN_ID = 5


ACTION_STR_TO_ID_LOOKUP = {
  NOOP_STR: NOOP_ID,
  UP_STR: UP_ID,
  RIGHT_STR: RIGHT_ID, 
  LEFT_STR: LEFT_ID,
  DOWN_STR: DOWN_ID
}

ACTION_ID_TO_STR_LOOKUP = {
  v : k for (k, v) in 
    ACTION_STR_TO_ID_LOOKUP.items()
  }

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
