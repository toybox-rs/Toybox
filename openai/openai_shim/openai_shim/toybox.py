import ctypes
import numpy as np

_lib_path = 'target/debug/libopenai.dylib'
_lib = ctypes.CDLL(_lib_path)

class WrapSimulator(ctypes.Structure):
    pass

class WrapState(ctypes.Structure):
    pass


# I don't know how actions will be issued, so let's have lots of options available
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

    def set_input(self, input_dir, button):
        self._set_default()

        # reset all directions
        if input_dir == LEFT:
            self.left = True
        elif input_dir == RIGHT:
            self.right = True
        elif input_dir == UP:
            self.up = True
        elif input_dir == DOWN:
            self.down = True
        else:
            assert False

        # reset buttons
        if button == BUTTON1:
            self.button1 = True
        if button == BUTTON2:
            self.button2 = True
        else:
            assert False
            

_lib.alloc_game_simulator.argtypes = [ctypes.c_char_p]
_lib.alloc_game_simulator.restype = ctypes.POINTER(WrapSimulator)

_lib.alloc_game_state.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.alloc_game_state.restype = ctypes.POINTER(WrapState)

_lib.frame_width.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.frame_width.restype = ctypes.c_int

_lib.frame_height.argtypes = [ctypes.POINTER(WrapSimulator)]
_lib.frame_height.restype = ctypes.c_int 


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

    def get_state(self):
        return self.__state

    def render_frame(self, sim):
        h = sim.get_frame_height()
        w = sim.get_frame_width()
        rgba = 1
        size = h * w * rgba
        frame = np.zeros(size)
        frame_ptr = frame.ctypes.data_as(ctypes.POINTER(ctypes.c_int))
        _lib.render_current_frame(frame_ptr, size, sim.get_simulator(), self.__state)
        return np.reshape(frame, (w,h,rgba))

class Toybox():

    def __init__(self, game_name):
        self.rsimulator = Simulator(game_name)
        self.rstate = State(self.rsimulator)
        # OpenAI state is a 4-frame sequence
        self.state = tuple([self.rstate.render_frame(self.rsimulator)] * 4)

    def get_state(self):
        return self.state

    def apply_action(self, action_input_obj):
        _lib.apply_action(self.rstate.get_state(), action_input_obj)
        new_frame = self.rstate.render_frame(self.rsimulator)
        self.state = (self.state[1], self.state[2], self.state[3], new_frame)
        return new_frame

    def get_score(self):
        return -1

    def __del__(self):
        pass


if __name__ == "__main__":
    with Simulator('amidar') as sim:
        with State(sim) as state:
            print('sim in main', sim)
            print('hahahahah')
            print('\tframe width:', sim.get_frame_width())
            print('\tframe height:', sim.get_frame_height())
            frame = state.render_frame(sim)
            from PIL import Image
            img = Image.fromarray(frame, 'RGB')
            img.save('my.png')
    tb = Toybox('breakout')
    tb.apply_action(Input())
        