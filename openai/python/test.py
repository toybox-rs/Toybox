#! /usr/bin/env python

import ctypes 

# {devhome}/toybox/openai/target/debug/libopenai.d exists, but 
# I am not sure where libopenai.so is supposed to live.
lib = ctypes.CDLL("./target/debug/libopenai.dylib")
# Can do this when LD_LIBRARY_PATH is set properly
#lib = ctypes.CDLL("libopenai.dylib")
print("It worked (1)")
print(str(lib.get_state()))
