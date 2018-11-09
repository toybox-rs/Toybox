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
    _lib_prefix = os.environ[lib_env_var] if lib_env_var in os.environ else '.'
    _lib_path_debug   = os.path.sep.join([_lib_prefix, 'target', 'debug', lib_dylib])
    _lib_path_release = os.path.sep.join([_lib_prefix, 'target', 'release', lib_dylib])

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

_lib.state_alloc.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.state_alloc.restype = ctypes.POINTER(WrapState)

_lib.simulator_frame_width.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.simulator_frame_width.restype = ctypes.c_int32

_lib.simulator_frame_height.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.simulator_frame_height.restype = ctypes.c_int32

_lib.state_lives.restype = ctypes.c_int32
_lib.state_score.restype = ctypes.c_int32
    
_lib.render_current_frame.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_bool, ctypes.c_void_p, ctypes.c_void_p]
 #(frame_ptr, size, sim.get_simulator(), self.__state)

_lib.to_json.argtypes = [ctypes.POINTER(WrapState)]
_lib.to_json.restype = ctypes.c_char_p

_lib.from_json.argtypes = [ctypes.POINTER(WrapSimulator), ctypes.c_char_p]
_lib.from_json.restype = ctypes.POINTER(WrapState)

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
