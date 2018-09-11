import ctypes

_lib_path = "target/debug/libopenai.dylib"
_lib = ctypes.CDLL(_lib_path)

def _get_game(game_name):
    # _lib.new_game.argtypes = [ctypes.c_char_p]
    return _lib.new_game(game_name)

class State(ctypes.Structure):
    _fields_ = []

class Toybox():

    def __init__(self, game_name):
        self.game = _get_game(ctypes.c_char_p(game_name.strip()))


if __name__ == "__main__":
    tb = Toybox("breakout")
    print("Toybox", tb, tb.game)
