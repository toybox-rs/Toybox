from collections import deque
import numpy as np
from PIL import Image
import os
import platform
import time
import json

try:
    from toybox_cffi import ffi, lib
except:
    # should be ModuleNotFoundError, but this is not available on the version of python on travis
    print('Global setup not found...trying local development install...')
    platform = platform.system() 
    LIB_NAME = "toybox_cffi"
    lib_env_var = 'LIBCTOYBOX'
    lib_dylib = 'lib{0}.dylib'.format(LIB_NAME)
    lib_so = 'lib{0}.so'.format(LIB_NAME)

    _lib_prefix = os.environ[lib_env_var] if lib_env_var in os.environ else '..'

    if platform == 'Darwin':
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
        raise Exception('Unsupported platform for development: %s' % platform)
    
    try:
        from cffi import FFI 
        ffi = FFI()
        with open(os.sep.join([_lib_prefix, 'target', 'ctoybox.h']), 'r') as f:
            # directives not supported!
            header = '\n'.join([line for line in f.readlines() if not line.startswith('#')])
            ffi.cdef(header)
        lib = ffi.dlopen(_lib_path)
    except Exception:
        raise Exception('Could not load libctoybox from path %s. ' % _lib_path 
        + """If you are on OSX, this may be due the relative path being different 
        from `target/(target|release)/libctoybox.dylib. If you are on Linux, try
        prefixing your call with `LD_LIBRARY_PATH=/path/to/library`.""")


class Input():
    """An input object represents a game controller having left, right, up, down, and two buttons.

    ALE mapping:
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

    _LEFT = "left"
    _RIGHT = "right"
    _UP = "up"
    _DOWN = "down"
    _BUTTON1 = "button1"
    _BUTTON2 = "button2"
    _NOOP = "noop"

    def __init__(self):
        self.reset()

    def reset(self):
        self.left = False
        self.right = False
        self.up = False
        self.down = False
        self.button1 = False
        self.button2 = False

    def __str__(self):
        return self.__dict__.__str__()

    def __repr__(self):
        return self.__dict__.__str__()

    def set_input(self, input_dir, button=_NOOP):
        input_dir = input_dir.lower()
        button = button.lower()

        # reset all directions
        if   input_dir == Input._NOOP:
            pass
        elif input_dir == Input._LEFT:
            self.left = True
        elif input_dir == Input._RIGHT:
            self.right = True
        elif input_dir == Input._UP:
            self.up = True
        elif input_dir == Input._DOWN:
            self.down = True
        else:
            print('input_dir:', input_dir)
            assert False

        # reset buttons
        if   button == Input._NOOP:
            pass
        elif button == Input._BUTTON1:
            self.button1 = True
        elif button == Input._BUTTON2:
            self.button2 = True
        else:
            assert False


def rust_str(result):
    txt = ffi.cast("char *", result) #.value.decode('UTF-8')
    txt = ffi.string(txt).decode('UTF-8')
    lib.free_str(result)
    return txt


def json_str(js):
    if type(js) is dict:
        js = json.dumps(js)
    elif type(js) is Input:
        js = json.dumps(js.__dict__)
    elif type(js) is not str:
        raise ValueError('Unknown json type: %s (only str and dict supported)' % type(js))
    return js

class Simulator(object):
    def __init__(self, game_name, sim=None):
        if sim is None:
            sim = lib.simulator_alloc(game_name.encode('utf-8'))
        # sim should be a pointer
        self.game_name = game_name
        self.__sim = sim 
        self.deleted = False

    def __del__(self):
        if not self.deleted:
            self.deleted = True
            lib.simulator_free(self.__sim)
            self.__sim = None

    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_value, traceback):
        self.__del__()
    
    def set_seed(self, value):
        lib.simulator_seed(self.__sim, value)

    def get_frame_width(self):
        return lib.simulator_frame_width(self.__sim)

    def get_frame_height(self):
        return lib.simulator_frame_height(self.__sim)

    def get_simulator(self):
        return self.__sim

    def new_game(self):
        return State(self)

    def state_from_json(self, js):
        state = lib.state_from_json(self.get_simulator(), json_str(js).encode('utf-8'))
        return State(self, state=state)

    def to_json(self):
        json_str = rust_str(lib.simulator_to_json(self.get_simulator()))
        return json.loads(str(json_str))

    def from_json(self, config_js):
        old_sim = self.__sim
        self.__sim = lib.simulator_from_json(self.get_simulator(), json_str(config_js).encode('utf-8'))
        del old_sim


class State(object):
    def __init__(self, sim, state=None):
        self.__state = state or lib.state_alloc(sim.get_simulator())
        self.game_name = sim.game_name
        self.deleted = False

    def __enter__(self):
        return self

    def __del__(self):
        if not self.deleted:
            self.deleted = True
            lib.state_free(self.__state)
            self.__state = None

    def __exit__(self, exc_type, exc_value, traceback):
        self.__del__()

    def get_state(self):
        assert(self.__state is not None)
        return self.__state
    
    def lives(self):
        return lib.state_lives(self.__state)

    def score(self):
        return lib.state_score(self.__state)

    def game_over(self):
        return self.lives() < 0

    def query_json(self, query, args="null"):
        txt = rust_str(lib.state_query_json(self.__state, json_str(query).encode('utf-8'), json_str(args).encode('utf-8')))
        try:
            out = json.loads(txt)
        except:
            raise ValueError(txt)
        return out

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
        frame_ptr = ffi.cast("uint8_t *", frame.ctypes.data)
        lib.render_current_frame(frame_ptr, size, False, sim.get_simulator(), self.__state)
        return np.reshape(frame, (h,w,rgba))

    def render_frame_rgb(self, sim):
        rgba_frame = self.render_frame_color(sim)
        return rgba_frame[:,:,:3]
    
    def render_frame_grayscale(self, sim):
        h = sim.get_frame_height()
        w = sim.get_frame_width()
        size = h * w 
        frame = np.zeros(size, dtype='uint8')
        frame_ptr = ffi.cast("uint8_t *", frame.ctypes.data)
        lib.render_current_frame(frame_ptr, size, True, sim.get_simulator(), self.__state)
        return np.reshape(frame, (h,w,1))

    def to_json(self):
        json_str = rust_str(lib.state_to_json(self.__state))
        return json.loads(str(json_str))

class Toybox(object):
    def __init__(self, game_name, grayscale=True, frameskip=0):
        self.game_name = game_name
        self.frames_per_action = frameskip+1
        self.rsimulator = Simulator(game_name)
        self.rstate = self.rsimulator.new_game()
        self.grayscale = grayscale
        self.deleted = False
        self.new_game()

    def new_game(self):
        old_state = self.rstate
        del old_state
        self.rstate = self.rsimulator.new_game()
        
    def get_height(self):
        return self.rsimulator.get_frame_height()

    def get_width(self):
        return self.rsimulator.get_frame_width()

    def get_legal_action_set(self):
        sim = self.rsimulator.get_simulator()
        txt = rust_str(lib.simulator_actions(sim))
        try:
            out = json.loads(txt)
        except:
            raise ValueError(txt)
        return out

    def apply_ale_action(self, action_int):
        """Takes an integer corresponding to an action, as specified in ALE and applies the action k times, where k is the sticky action constant stored in self.frames_per_action.
        """      
        # implement frameskip(k) by sending the action (k+1) times every time we have an action.
        for _ in range(self.frames_per_action):
            if not lib.state_apply_ale_action(self.rstate.get_state(), action_int):
                raise ValueError("Expected to apply action, but failed: {0}".format(action_int))

    def apply_action(self, action_input_obj):
        """Takes an Input 
        """
        # implement frameskip(k) by sending the action (k+1) times every time we have an action.
        for _ in range(self.frames_per_action):
            js = json_str(action_input_obj).encode('UTF-8') 
            lib.state_apply_action(self.rstate.get_state(), 
                                   ffi.new("char []", js))
    
    def get_state(self):
        return self.rstate.render_frame(self.rsimulator, self.grayscale)
    
    def set_seed(self, seed):
        self.rsimulator.set_seed(seed)

    def save_frame_image(self, path, grayscale=False):
        img = None
        if grayscale:
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

    def state_to_json(self):
        return self.rstate.to_json()

    def to_state_json(self):
        return self.rstate.to_json()

    def config_to_json(self):
        return self.rsimulator.to_json()

    def write_state_json(self, js):
        old_state = self.rstate
        del old_state
        self.rstate = self.rsimulator.state_from_json(js)

    def write_config_json(self, config_js):
        # from_json replaces simulator!
        self.rsimulator.from_json(config_js)
        # new_game replaces state!
        self.new_game()

    def query_state_json(self, query, args="null"): 
        return self.rstate.query_json(query, args)

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
        print(tb.config_to_json())
        print(tb.state_to_json())
