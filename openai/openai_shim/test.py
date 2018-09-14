import ctypes 
import os

dll_path = "target/debug/libopenai.dylib"
assert(os.path.exists(dll_path))

lib = ctypes.CDLL(dll_path)
# Can do this when LD_LIBRARY_PATH is set properly
#lib = ctypes.CDLL("libopenai.dylib")
# note: I think this is DYLD_LIBRARY_PATH on OSX
for name in dir(lib):
    print(name, lib.__getattribute__(name))

class Game(ctypes.Structure):
    _fields_ = [('done', ctypes.c_bool)]

    def __str__(self):
        if self.done:
            return "Game Over!"
        else:
            return "In play."

lib.get_state.argtypes = (None)
lib.get_state.restype = str

print(lib.get_state())
print(lib.is_done)
print(Game)

lib.is_done.argtypes = (ctypes.POINTER(Game))
lib.is_done.restype = bool 