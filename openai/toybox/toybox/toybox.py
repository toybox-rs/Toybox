from collections import deque
import ctypes
import numpy as np
from PIL import Image
import os
import platform
import time
import json

platform = platform.system() 
libopenai = 'LIBOPENAI'

if platform == 'Darwin':
    _lib_prefix = os.environ[libopenai] if libopenai in os.environ else '.'
    _lib_path_debug   = os.path.sep.join([_lib_prefix, 'target', 'debug', 'libopenai.dylib'])
    _lib_path_release = os.path.sep.join([_lib_prefix, 'target', 'release', 'libopenai.dylib'])

    _lib_ts_release = os.stat(_lib_path_release).st_birthtime \
        if os.path.exists(_lib_path_release) else 0
    _lib_ts_debug   = os.stat(_lib_path_debug).st_birthtime \
        if os.path.exists(_lib_path_debug) else 0
        
    if (not (_lib_ts_debug or _lib_ts_release)):
        raise OSError('libopenai.dylib not found on this machine')

    _lib_path = _lib_path_debug if _lib_ts_debug > _lib_ts_release else _lib_path_release
    print(_lib_path)

elif platform == 'Linux':
    _lib_path = 'libopenai.so'
    
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

class Simulator(object):
    def __init__(self, game_name):
        sim = _lib.simulator_alloc(game_name.encode('utf-8'))
        # sim should be a pointer
        #self.__sim = ctypes.pointer(ctypes.c_int(sim))
        self.__sim = sim 
        self.__width = _lib.simulator_frame_width(sim)
        self.__height = _lib.simulator_frame_height(sim)
        self.deleted = False

    def __del__(self):
        if not self.deleted:
            self.deleted = True
            _lib.simulator_free(self.__sim)
            self.__sim = None

    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_value, traceback):
        self.__del__()
    
    def set_seed(self, value):
        _lib.simulator_seed(self.__sim, value)

    def get_frame_width(self):
        return self.__width

    def get_frame_height(self):
        return self.__height

    def get_simulator(self):
        return self.__sim

    def new_game(self):
        return State(self)

    def from_json(self, js):
        if type(js) is dict:
            js = json.dumps(js)
        elif type(js) is not str:
            raise ValueError('Unknown json type: %s (only str and dict supported)' % type(js))
        state = _lib.from_json(self.get_simulator(), js.encode('utf-8'))
        return State(self, state=state)



class State(object):
    def __init__(self, sim, state=None):
        self.__state = state or _lib.state_alloc(sim.get_simulator())
        self.deleted = False

    def __enter__(self):
        return self

    def __del__(self):
        if not self.deleted:
            self.deleted = True
            _lib.state_free(self.__state)
            self.__state = None

    def __exit__(self, exc_type, exc_value, traceback):
        self.__del__()

    def get_state(self):
        return self.__state
    
    def lives(self):
        return _lib.state_lives(self.__state)

    def score(self):
        return _lib.state_score(self.__state)

    def game_over(self):
        return self.lives() == 0

    def breakout_bricks_remaining(self):
        return _lib.breakout_bricks_remaining(self.__state)
    
    def breakout_channel_count(self):
        return len(self.breakout_channels())
    
    def breakout_num_columns(self):
        return _lib.breakout_num_columns(self.__state)

    def breakout_num_rows(self):
        return _lib.breakout_num_rows(self.__state)

    def amidar_num_tiles_unpainted(self):
        return _lib.amidar_num_tiles_unpainted(self.__state)
    
    def amidar_player_tile(self):
        x = _lib.amidar_player_tile_x(self.__state)
        y = _lib.amidar_player_tile_y(self.__state)
        return (x,y)
    
    def amidar_num_enemies(self):
        return _lib.amidar_num_enemies(self.__state)
    
    def amidar_jumps_remaining(self):
        return _lib.amidar_jumps_remaining(self.__state)

    def amidar_regular_mode(self):
        return _lib.amidar_regular_mode(self.__state)

    def amidar_jump_mode(self):
        return _lib.amidar_jump_mode(self.__state)

    def amidar_chase_mode(self):
        return _lib.amidar_chase_mode(self.__state)

    def amidar_enemy_tiles(self):
        num_enemies = self.amidar_num_enemies()
        out = []
        for eid in range(num_enemies):
            x = _lib.amidar_enemy_tile_x(self.__state, eid)
            y = _lib.amidar_enemy_tile_y(self.__state, eid)
            out.append((x,y))
        return out

    def breakout_channels(self):
        NC = self.breakout_num_columns()
        arr = np.zeros(NC, dtype='int32')
        found = _lib.breakout_channels(self.__state, arr.ctypes.data_as(ctypes.POINTER(ctypes.c_int32)), NC)
        assert(found >= 0)
        return arr.tolist()[:found]
            
    def render_frame(self, sim, grayscale=True):
        if grayscale:
            return self.render_frame_grayscale(sim)
        else:
            return self.render_frame_color(sim)

    def render_frame_color(self, sim):
        h = sim.get_frame_height()
        w = sim.get_frame_width()
        rgba = 4
        size = h * w  * rgba
        frame = np.zeros(size, dtype='uint8')
        frame_ptr = frame.ctypes.data_as(ctypes.POINTER(ctypes.c_uint8))
        _lib.render_current_frame(frame_ptr, size, False, sim.get_simulator(), self.__state)
        return np.reshape(frame, (h,w,rgba))

    def render_frame_rgb(self, sim):
        rgba_frame = self.render_frame_color(sim)
        return rgba_frame[:,:,:3]
    
    def render_frame_grayscale(self, sim):
        h = sim.get_frame_height()
        w = sim.get_frame_width()
        size = h * w 
        frame = np.zeros(size, dtype='uint8')
        frame_ptr = frame.ctypes.data_as(ctypes.POINTER(ctypes.c_uint8))
        _lib.render_current_frame(frame_ptr, size, True, sim.get_simulator(), self.__state)
        return np.reshape(frame, (h,w,1))

    def to_json(self):
        json_str = _lib.to_json(self.__state).decode('utf-8')
        return json.loads(str(json_str))

class Toybox(object):
    def __init__(self, game_name, grayscale=True, frameskip=0, k=4):
        self.frames_per_action = frameskip+1
        self.rsimulator = Simulator(game_name)
        self.rstate = State(self.rsimulator)
        self.grayscale = grayscale
        self.k = k 
        # OpenAI state is a 4-frame sequence
        self.state = None
        self._set_state(k)
        self.deleted = False

    def _set_state(self, k):
        self.state = deque([], maxlen=k)
        frame = self.rstate.render_frame(self.rsimulator, self.grayscale)
        for _ in range(k):
            self.state.append(frame)
        assert (self.state)

    def get_state(self):
        assert(self.state)
        return self.state

    def new_game(self):
        old_state = self.rstate
        del old_state
        self.rstate = self.rsimulator.new_game()
        self._set_state(self.k)
        
    def get_height(self):
        return self.rsimulator.get_frame_height()

    def get_width(self):
        return self.rsimulator.get_frame_width()

    def apply_action(self, action_input_obj):
        # implement frameskip(k) by sending the action (k+1) times every time we have an action.
        for _ in range(self.frames_per_action):
            _lib.state_apply_action(self.rstate.get_state(), ctypes.byref(action_input_obj))
        new_frame = self.rstate.render_frame(self.rsimulator, self.grayscale)
        self.state.append(new_frame)
        return new_frame
    
    def set_seed(self, seed):
        self.rsimulator.set_seed(seed)

    def save_frame_image(self, path):
        img = None
        if self.grayscale:
            img = Image.fromarray(self.rstate.render_frame_grayscale(self.rsimulator), 'L') 
        else:
            img = Image.fromarray(self.rstate.render_frame_color(self.rsimulator), 'RGBA')
        img.save(path, format='png')

    def get_rgb_frame(self):
        return self.rstate.render_frame_rgb(self.rsimulator)

    def get_score(self):
        return self.rstate.score()
    
    def get_lives(self):
        return self.rstate.lives()
    
    def game_over(self):
        return self.rstate.game_over()

    def to_json(self):
        return self.rstate.to_json()

    def from_json(self, js):
        return self.rsimulator.from_json(js)

    def predicate_met(self, pred): 
        return False

    def __del__(self):
        if not self.deleted:
            self.deleted = True
            del self.rstate
            self.rstate = None
            del self.rsimulator
            self.rsimulator = None
    
    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_value, traceback):
        self.__del__()


if __name__ == "__main__":
    with Toybox('amidar') as tb:
        print(tb.to_json())