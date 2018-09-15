from openai_shim.openai_shim.env import BreakoutEnv
import itertools

NOOP_STR = 'NOOP' 
RIGHT_STR = 'RIGHT'
LEFT_STR = 'LEFT'

NOOP_ID = 0
RIGHT_ID = 3
LEFT_ID = 4


ACTION_STR_TO_ID_LOOKUP = {
  NOOP_STR: NOOP_ID,
  RIGHT_STR: RIGHT_ID, 
  LEFT_STR: LEFT_ID
}

ACTION_ID_TO_STR_LOOKUP = {
  v : k for (k, v) in 
    ACTION_STR_TO_ID_LOOKUP.items()
  }
