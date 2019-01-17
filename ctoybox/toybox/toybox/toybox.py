from collections import deque
import ctypes
import numpy as np
from PIL import Image
import os
import platform
import time
import json

from toybox.clib import _lib, Input, NOOP, LEFT, RIGHT, UP, DOWN, BUTTON1, BUTTON2

def json_str(js):
    if type(js) is dict:
        js = json.dumps(js)
    elif type(js) is not str:
        raise ValueError('Unknown json type: %s (only str and dict supported)' % type(js))
    return js

class Simulator(object):
    def __init__(self, game_name):
        sim = _lib.simulator_alloc(game_name.encode('utf-8'))
        # sim should be a pointer
        #self.__sim = ctypes.pointer(ctypes.c_int(sim))
        self.game_name = game_name
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
        state = _lib.from_json(self.get_simulator(), json_str(js).encode('utf-8'))
        return State(self, state=state)

    def from_config_json(self, config_js, state_js):
        state = _lib.from_config_json(self.get_simulator(), 
                    json_str(config_js).encode('utf-8'),
                    json_str(state_js).encode('utf-8'))
        return State(self, state=state)


class State(object):
    def __init__(self, sim, state=None):
        self.__state = state or _lib.state_alloc(sim.get_simulator())
        self.game_name = sim.game_name
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

    def breakout_brick_live_by_index(self, index):
        assert(self.game_name == 'breakout')
        return _lib.breakout_brick_live_by_index(self.__state, index)

    def breakout_bricks_remaining(self):
        assert(self.game_name == 'breakout')
        return _lib.breakout_bricks_remaining(self.__state)
    
    def breakout_channel_count(self):
        assert(self.game_name == 'breakout')
        return len(self.breakout_channels())
    
    def breakout_num_columns(self):
        assert(self.game_name == 'breakout')
        return _lib.breakout_num_columns(self.__state)

    def breakout_num_rows(self):
        assert(self.game_name == 'breakout')
        return _lib.breakout_num_rows(self.__state)

    def breakout_channels(self):
        assert(self.game_name == 'breakout')
        NC = self.breakout_num_columns()
        arr = np.zeros(NC, dtype='int32')
        found = _lib.breakout_channels(self.__state, arr.ctypes.data_as(ctypes.POINTER(ctypes.c_int32)), NC)
        assert(found >= 0)
        return arr.tolist()[:found]

    def amidar_num_tiles_unpainted(self):
        assert(self.game_name == 'amidar')
        return _lib.amidar_num_tiles_unpainted(self.__state)
    
    def amidar_player_tile(self):
        assert(self.game_name == 'amidar')
        x = _lib.amidar_player_tile_x(self.__state)
        y = _lib.amidar_player_tile_y(self.__state)
        return (x,y)
    
    def amidar_num_enemies(self):
        assert(self.game_name == 'amidar')
        return _lib.amidar_num_enemies(self.__state)
    
    def amidar_jumps_remaining(self):
        assert(self.game_name == 'amidar')
        return _lib.amidar_jumps_remaining(self.__state)

    def amidar_regular_mode(self):
        assert(self.game_name == 'amidar')
        return _lib.amidar_regular_mode(self.__state)

    def amidar_jump_mode(self):
        assert(self.game_name == 'amidar')
        return _lib.amidar_jump_mode(self.__state)

    def amidar_chase_mode(self):
        assert(self.game_name == 'amidar')
        return _lib.amidar_chase_mode(self.__state)

    def amidar_enemy_tiles(self):
        assert(self.game_name == 'amidar')
        num_enemies = self.amidar_num_enemies()
        out = []
        for eid in range(num_enemies):
            x = _lib.amidar_enemy_tile_x(self.__state, eid)
            y = _lib.amidar_enemy_tile_y(self.__state, eid)
            out.append((x,y))
        return out

    def amidar_enemy_caught(self, eid):
        assert(self.game_name == 'amidar')
        return _lib.amidar_enemy_caught(self.__state, eid)

    def amidar_any_enemy_caught(self, eid):
        assert(self.game_name == 'amidar')
        num_enemies = self.amidar_num_enemies()
        return any(self.amidar_enemy_caught(eid) for eid in range(num_enemies))

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

    def config_to_json(self):
        json_str = _lib.config_to_json(self.__state).decode('utf-8')
        return json.loads(str(json_str))

class Toybox(object):
    def __init__(self, game_name, grayscale=True, frameskip=0):
        self.game_name = game_name
        self.frames_per_action = frameskip+1
        self.rsimulator = Simulator(game_name)
        self.rstate = State(self.rsimulator)
        self.grayscale = grayscale
        self.deleted = False

    def new_game(self):
        old_state = self.rstate
        del old_state
        self.rstate = self.rsimulator.new_game()
        
    def get_height(self):
        return self.rsimulator.get_frame_height()

    def get_width(self):
        return self.rsimulator.get_frame_width()

    def get_legal_action_set(self):
        MAX_ALE_ACTION = 17
        # TODO, not hardcode 18?
        sim = self.rsimulator.get_simulator()
        return [x for x in range(MAX_ALE_ACTION+1) if _lib.simulator_is_legal_action(sim, x)]

    def apply_ale_action(self, action_int):      
        # implement frameskip(k) by sending the action (k+1) times every time we have an action.
        for _ in range(self.frames_per_action):
            if not _lib.state_apply_ale_action(self.rstate.get_state(), action_int):
                raise ValueError("Expected to apply action, but failed: {0}".format(action_int))
        return self.rstate.render_frame(self.rsimulator, self.grayscale)

    def apply_action(self, action_input_obj):
        # implement frameskip(k) by sending the action (k+1) times every time we have an action.
        for _ in range(self.frames_per_action):
            _lib.state_apply_action(self.rstate.get_state(), ctypes.byref(action_input_obj))
        return self.rstate.render_frame(self.rsimulator, self.grayscale)
    
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

    def to_json(self):
        return self.rstate.to_json()

    def config_to_json(self):
        return self.rstate.config_to_json()

    def from_json(self, js):
        return self.rsimulator.from_json(js)

    def write_json(self, js):
        old_state = self.rstate
        del old_state
        self.rstate = self.from_json(js)

    def write_config_json(self, config_js, state_js):
        old_state = self.rstate
        del old_state
        self.rstate = self.rsimulator.from_config_json(config_js, state_js)

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
