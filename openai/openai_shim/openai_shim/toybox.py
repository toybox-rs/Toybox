import ctypes
import numpy as np

_lib_path = 'target/debug/libopenai.dylib'
_lib = ctypes.CDLL(_lib_path)

class WrapSimulator(ctypes.Structure):
    pass

class WrapState(ctypes.Structure):
    pass

_lib.alloc_game_simulator.argtypes = [ctypes.c_char_p]
_lib.alloc_game_simulator.restype = ctypes.POINTER(WrapSimulator)

_lib.alloc_game_state.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.alloc_game_state.restype = ctypes.POINTER(WrapState)

_lib.frame_width.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.frame_width.restype = ctypes.c_int

_lib.frame_height.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.frame_height.restype = ctypes.c_int 


def _get_frame(game):
    return None

def _to_action(action):
    return None

def _apply_action(game, action):
    return None

def _get_score(game):
    return None


class Simulator(object):
    def __init__(self, game_name):
        sim = _lib.alloc_game_simulator(game_name.encode('utf-8'))
        # sim should be a pointer
        #self.__sim = ctypes.pointer(ctypes.c_int(sim))
        self.__sim = sim 
        print('sim', self.__sim)
        self.__width = _lib.frame_width(sim)
        self.__height = _lib.frame_height(sim)

    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_value, traceback):
        _lib.free_game_simulator(self.__sim)
        self.__sim = None

    def get_frame_width(self):
        return self.__width

    def get_frame_height(self):
        return self.__height

    def get_simulator(self):
        return self.__sim


class State(object):
    def __init__(self, sim):
        self.__state = _lib.alloc_game_state(sim.get_simulator())

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
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
    # sim = _lib.alloc_game_simulator(bytes('breakout', 'utf-8'))
    # print(sim)
    # print('frame width', sim.get_frame_width())
    # _lib.free_game_simulator(sim)
    with Simulator('breakout') as sim:
        with State(sim) as state:
            print('sim in main', sim)
            print('hahahahah')
            print('\tframe width:', sim.get_frame_width())
            print('\tframe height:', sim.get_frame_height())