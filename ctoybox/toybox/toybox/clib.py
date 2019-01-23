from collections import deque
import ctypes
import numpy as np
from PIL import Image
import os
import platform
import time
import json

platform = platform.system() 
lib_env_var = 'LIBCTOYBOX'
lib_dylib = 'libctoybox.dylib'
lib_so = 'libctoybox.so'


if platform == 'Darwin':
    _lib_prefix = os.environ[lib_env_var] if lib_env_var in os.environ else '..'
    _lib_path_debug   = os.path.sep.join([_lib_prefix, 'target', 'debug', lib_dylib])
    _lib_path_release = os.path.sep.join([_lib_prefix, 'target', 'release', lib_dylib])
    print('Looking for toybox lib in\n\t%s\nor\n\t%s' % (
        _lib_path_debug,
        _lib_path_release
    ))

    _lib_ts_release = os.stat(_lib_path_release).st_birthtime \
        if os.path.exists(_lib_path_release) else 0
    _lib_ts_debug   = os.stat(_lib_path_debug).st_birthtime \
        if os.path.exists(_lib_path_debug) else 0
        
    if (not (_lib_ts_debug or _lib_ts_release)):
        raise OSError('%s not found on this machine' % lib_dylib)

    _lib_path = _lib_path_debug if _lib_ts_debug > _lib_ts_release else _lib_path_release
    print(_lib_path)

elif platform == 'Linux':
    _lib_path = lib_so
    
else:
    raise Exception('Unsupported platform: %s' % platform)


try:
    _lib = ctypes.CDLL(_lib_path)
except Exception:
    raise Exception('Could not load libopenai from path %s.' % _lib_path 
    + """If you are on OSX, this may be due the relative path being different 
    from `target/(target|release)/libopenai.dylib. If you are on Linux, try
    prefixing your call with `LD_LIBRARY_PATH=/path/to/library`.""")

class WrapSimulator(ctypes.Structure):
    pass

class WrapState(ctypes.Structure):
    pass


# I don't know how actions will be issued, so let's have lots of options available
NOOP = 'noop'
LEFT = "left"
RIGHT = "right"
UP = "up"
DOWN = "down"
BUTTON1 = "button1"
BUTTON2 = "button2"

class Input(ctypes.Structure):
    _fields_ = [(LEFT, ctypes.c_bool), 
                (RIGHT, ctypes.c_bool),
                (UP, ctypes.c_bool),
                (DOWN, ctypes.c_bool),
                (BUTTON1, ctypes.c_bool),
                (BUTTON2, ctypes.c_bool)]

    def _set_default(self):
        self.left = False
        self.right = False
        self.up = False
        self.down = False
        self.button1 = False
        self.button2 = False

    """
        ALE_ACTION_MEANING = {
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
    """
    def set_ale(self, num):
        if num == 0:
            pass
        elif num == 1:
            self.button1 = True
        elif num == 2:
            self.up = True
        elif num == 3:
            self.right = True
        elif num == 4:
            self.left = True
        elif num == 5:
            self.down = True
        elif num == 6:
            self.up = True
            self.right = True
        elif num == 7:
            self.up = True
            self.left = True
        elif num == 8:
            self.down = True
            self.right = True
        elif num == 9:
            self.down = True
            self.left = True
        elif num == 10:
            self.up = True
            self.button1 = True
        elif num == 11:
            self.right = True
            self.button1 = True
        elif num == 12:
            self.left = True
            self.button1 = True
        elif num == 13:
            self.down = True
            self.button1 = True
        elif num == 14:
            self.up = True
            self.right = True
            self.button1 = True
        elif num == 15:
            self.up = True
            self.left = True
            self.button1 = True
        elif num == 16:
            self.down = True
            self.right = True
            self.button1 = True
        elif num == 17:
            self.down = True
            self.left = True
            self.button1 = True


    def set_input(self, input_dir, button=NOOP):
        self._set_default()
        input_dir = input_dir.lower()
        button = button.lower()

        # reset all directions
        if input_dir == NOOP:
            pass
        elif input_dir == LEFT:
            self.left = True
        elif input_dir == RIGHT:
            self.right = True
        elif input_dir == UP:
            self.up = True
        elif input_dir == DOWN:
            self.down = True
        else:
            print('input_dir:', input_dir)
            assert False

        # reset buttons
        if button == NOOP:
            pass
        elif button == BUTTON1:
            self.button1 = True
        elif button == BUTTON2:
            self.button2 = True
        else:
            assert False
            

_lib.simulator_alloc.argtypes = [ctypes.c_char_p]
_lib.simulator_alloc.restype = ctypes.POINTER(WrapSimulator)

_lib.simulator_seed.argtypes = [ctypes.POINTER(WrapSimulator), ctypes.c_uint]
_lib.simulator_seed.restype = None

_lib.simulator_is_legal_action.argtypes = [ctypes.POINTER(WrapSimulator), ctypes.c_int32]
_lib.simulator_is_legal_action.restype = ctypes.c_bool

_lib.state_alloc.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.state_alloc.restype = ctypes.POINTER(WrapState)

_lib.state_query_json.argtypes = [ctypes.POINTER(WrapState), ctypes.c_char_p]
_lib.state_query_json.restype = ctypes.c_char_p

_lib.state_apply_ale_action.argtypes = [ctypes.POINTER(WrapState), ctypes.c_int32]
_lib.state_apply_ale_action.restype = ctypes.c_bool

_lib.state_apply_action.argtypes = [ctypes.POINTER(WrapState), ctypes.POINTER(Input)]
_lib.state_apply_action.restype = None

_lib.simulator_frame_width.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.simulator_frame_width.restype = ctypes.c_int32

_lib.simulator_frame_height.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.simulator_frame_height.restype = ctypes.c_int32

_lib.state_lives.restype = ctypes.c_int32
_lib.state_score.restype = ctypes.c_int32
    
_lib.render_current_frame.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_bool, ctypes.c_void_p, ctypes.c_void_p]
 #(frame_ptr, size, sim.get_simulator(), self.__state)

_lib.state_to_json.argtypes = [ctypes.POINTER(WrapState)]
_lib.state_to_json.restype = ctypes.c_char_p

_lib.state_from_json.argtypes = [ctypes.POINTER(WrapSimulator), ctypes.c_char_p]
_lib.state_from_json.restype = ctypes.POINTER(WrapState)

_lib.simulator_to_json.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.simulator_to_json.restype = ctypes.c_char_p

_lib.simulator_from_json.argtypes = [ctypes.POINTER(WrapSimulator), ctypes.c_char_p]
_lib.simulator_from_json.restype = ctypes.POINTER(WrapSimulator)

_lib.breakout_brick_live_by_index.argtypes = [ctypes.POINTER(WrapState), ctypes.c_size_t]
_lib.breakout_brick_live_by_index.restype = ctypes.c_bool

_lib.breakout_bricks_remaining.argtypes = [ctypes.POINTER(WrapState)]
_lib.breakout_bricks_remaining.restype = ctypes.c_int32

_lib.breakout_num_rows.argtypes = [ctypes.POINTER(WrapState)]
_lib.breakout_num_rows.restype = ctypes.c_int32

_lib.breakout_num_columns.argtypes = [ctypes.POINTER(WrapState)]
_lib.breakout_num_columns.restype = ctypes.c_int32

_lib.breakout_channels.argtypes = [ctypes.POINTER(WrapState), ctypes.c_void_p, ctypes.c_size_t]
_lib.breakout_channels.restype = ctypes.c_ssize_t

_lib.amidar_num_tiles_unpainted.argtypes = [ctypes.POINTER(WrapState)]
_lib.amidar_num_tiles_unpainted.restype = ctypes.c_int32

_lib.amidar_num_enemies.argtypes = [ctypes.POINTER(WrapState)]
_lib.amidar_num_enemies.restype = ctypes.c_int32

_lib.amidar_jumps_remaining.argtypes = [ctypes.POINTER(WrapState)]
_lib.amidar_jumps_remaining.restype = ctypes.c_int32

_lib.amidar_regular_mode.argtypes = [ctypes.POINTER(WrapState)]
_lib.amidar_regular_mode.restype = ctypes.c_bool

_lib.amidar_chase_mode.argtypes = [ctypes.POINTER(WrapState)]
_lib.amidar_chase_mode.restype = ctypes.c_bool

_lib.amidar_jump_mode.argtypes = [ctypes.POINTER(WrapState)]
_lib.amidar_jump_mode.restype = ctypes.c_bool

_lib.amidar_player_tile_x.argtypes = [ctypes.POINTER(WrapState)]
_lib.amidar_player_tile_x.restype = ctypes.c_int32

_lib.amidar_player_tile_y.argtypes = [ctypes.POINTER(WrapState)]
_lib.amidar_player_tile_y.restype = ctypes.c_int32

_lib.amidar_enemy_tile_x.argtypes = [ctypes.POINTER(WrapState), ctypes.c_int32]
_lib.amidar_enemy_tile_x.restype = ctypes.c_int32

_lib.amidar_enemy_tile_y.argtypes = [ctypes.POINTER(WrapState), ctypes.c_int32]
_lib.amidar_enemy_tile_y.restype = ctypes.c_int32

_lib.amidar_enemy_caught.argtypes = [ctypes.POINTER(WrapState), ctypes.c_int32]
_lib.amidar_enemy_caught.restype = ctypes.c_bool
