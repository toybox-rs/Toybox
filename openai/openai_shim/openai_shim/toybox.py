import ctypes
import numpy as np

_lib_path = "target/debug/libopenai.dylib"
_lib = ctypes.CDLL(_lib_path)

_lib.alloc_game_simulator.argtypes = [ctypes.c_char_p]
_lib.alloc_game_simulator.restype = ctypes.c_void_p

_lib.frame_width.argtypes = [ctypes.c_void_p]
_lib.frame_width.restype = ctypes.c_int8

_lib.frame_height.argtypes = [ctypes.c_void_p]
_lib.frame_height.restype = ctypes.c_int8


def _get_frame(game):
    _lib.get_frame.argtypes = [ctypes.c_void_p]
    # _lib.get_frame.restype = [ctypes.c_]
    

def _to_action(action):
    pass

def _apply_action(game, action):
    pass

def _get_score(game):
    pass


class Simulator(object):
    def __init__(self, game_name):
        as_utf8 = game_name.encode('utf-8')
        cstring_game_name = ctypes.create_string_buffer(as_utf8)
        sim = _lib.alloc_game_simulator(cstring_game_name)
        # sim should be a pointer
        self.__sim = sim
        self.__width = _lib.frame_width(self.__sim)
        # self.__height = _lib.frame_width(self.__sim)

    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_value, traceback):
        _lib.free_game_simulator.argtypes = [ctypes.c_void_p]
        _lib.free_game_simulator(self.__sim)
        self.__sim = None

    def get_board_width(self):
        return self.__width

    def get_board_height(self):
        return self.__height


class State(object):
    def __init__(self, sim):
        _lib.alloc_game_state.argtypes = [ctypes.c_void_p]
        _lib.alloc_game_state.restype = ctypes.c_void_p
        self.__state = _lib.alloc_game_state(sim.__sim)

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        _lib.free_game_state.argtypes = [ctypes.c_void_p]
        _lib.free_game_state(self.__state)
        self.__state = None

class Toybox():

    def __init__(self, game_name):
        self.game = _get_game()
        # OpenAI state is a 4-frame sequence
        self.state = tuple([_get_frame(self.game)] * 4)

    def get_state(self):
        return self.state

    def apply_action(self, action):
        action = _to_action(action)
        new_frame = _apply_action(self.game, action)
        self.state = (self.state[1], self.state[2], self.state[3], new_frame)
        return new_frame

    def get_score(self):
        return _get_score(self.game)


if __name__ == "__main__":
    with Simulator("breakout") as sim:
        print(sim)
