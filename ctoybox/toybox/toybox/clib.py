# from collections import deque
# import ctypes
# import numpy as np
# from PIL import Image
# import os
# import platform
# import time
# import json

# platform = platform.system() 
# lib_env_var = 'LIBCTOYBOX'
# lib_dylib = 'libctoybox.dylib'
# lib_so = 'libctoybox.so'


# if platform == 'Darwin':
#     _lib_prefix = os.environ[lib_env_var] if lib_env_var in os.environ else '..'
#     _lib_path_debug   = os.path.sep.join([_lib_prefix, 'target', 'debug', lib_dylib])
#     _lib_path_release = os.path.sep.join([_lib_prefix, 'target', 'release', lib_dylib])
#     print('Looking for toybox lib in\n\t%s\nor\n\t%s' % (
#         _lib_path_debug,
#         _lib_path_release
#     ))

#     _lib_ts_release = os.stat(_lib_path_release).st_birthtime \
#         if os.path.exists(_lib_path_release) else 0
#     _lib_ts_debug   = os.stat(_lib_path_debug).st_birthtime \
#         if os.path.exists(_lib_path_debug) else 0
        
#     if (not (_lib_ts_debug or _lib_ts_release)):
#         raise OSError('%s not found on this machine' % lib_dylib)

#     _lib_path = _lib_path_debug if _lib_ts_debug > _lib_ts_release else _lib_path_release
#     print(_lib_path)

# elif platform == 'Linux':
#     _lib_path = lib_so
    
# else:
#     raise Exception('Unsupported platform: %s' % platform)


# try:
#     _lib = ctypes.CDLL(_lib_path)
# except Exception:
#     raise Exception('Could not load libopenai from path %s.' % _lib_path 
#     + """If you are on OSX, this may be due the relative path being different 
#     from `target/(target|release)/libopenai.dylib. If you are on Linux, try
#     prefixing your call with `LD_LIBRARY_PATH=/path/to/library`.""")

# class WrapSimulator(ctypes.Structure):
#     pass

# class WrapState(ctypes.Structure):
#     pass


# # I don't know how actions will be issued, so let's have lots of options available
# NOOP = 'noop'
# LEFT = "left"
# RIGHT = "right"
# UP = "up"
# DOWN = "down"
# BUTTON1 = "button1"
# BUTTON2 = "button2"

            

# _lib.simulator_alloc.argtypes = [ctypes.c_char_p]
# _lib.simulator_alloc.restype = ctypes.POINTER(WrapSimulator)

# _lib.simulator_seed.argtypes = [ctypes.POINTER(WrapSimulator), ctypes.c_uint]
# _lib.simulator_seed.restype = None

# _lib.simulator_is_legal_action.argtypes = [ctypes.POINTER(WrapSimulator), ctypes.c_int32]
# _lib.simulator_is_legal_action.restype = ctypes.c_bool

# _lib.simulator_actions.argtypes = [ctypes.POINTER(WrapSimulator)]
# _lib.simulator_actions.restype = ctypes.c_void_p

# _lib.state_alloc.argtypes = [ctypes.POINTER(WrapSimulator)]
# _lib.state_alloc.restype = ctypes.POINTER(WrapState)

# _lib.free_str.argtypes = [ctypes.c_void_p]
# _lib.free_str.restype = None

# _lib.state_query_json.argtypes = [ctypes.POINTER(WrapState), ctypes.c_char_p, ctypes.c_char_p]
# _lib.state_query_json.restype = ctypes.c_void_p

# _lib.state_apply_ale_action.argtypes = [ctypes.POINTER(WrapState), ctypes.c_int32]
# _lib.state_apply_ale_action.restype = ctypes.c_bool

# _lib.state_apply_action.argtypes = [ctypes.POINTER(WrapState), ctypes.POINTER(Input)]
# _lib.state_apply_action.restype = None

# _lib.simulator_frame_width.argtypes = [ctypes.POINTER(WrapSimulator)]
# _lib.simulator_frame_width.restype = ctypes.c_int32

# _lib.simulator_frame_height.argtypes = [ctypes.POINTER(WrapSimulator)]
# _lib.simulator_frame_height.restype = ctypes.c_int32

# _lib.state_lives.restype = ctypes.c_int32
# _lib.state_score.restype = ctypes.c_int32
    
# _lib.render_current_frame.argtypes = [ctypes.c_void_p, ctypes.c_size_t, ctypes.c_bool, ctypes.c_void_p, ctypes.c_void_p]
#  #(frame_ptr, size, sim.get_simulator(), self.__state)

# _lib.state_to_json.argtypes = [ctypes.POINTER(WrapState)]
# _lib.state_to_json.restype = ctypes.c_void_p

# _lib.state_from_json.argtypes = [ctypes.POINTER(WrapSimulator), ctypes.c_char_p]
# _lib.state_from_json.restype = ctypes.POINTER(WrapState)

# _lib.simulator_to_json.argtypes = [ctypes.POINTER(WrapSimulator)]
# _lib.simulator_to_json.restype = ctypes.c_void_p

# _lib.simulator_from_json.argtypes = [ctypes.POINTER(WrapSimulator), ctypes.c_char_p]
# _lib.simulator_from_json.restype = ctypes.POINTER(WrapSimulator)
