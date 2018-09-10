#! /usr/bin/env python

import ctypes 

# {devhome}/toybox/openai/target/debug/libopenai.d exists, but 
# I am not sure where libopenai.so is supposed to live.
#ctypes.CDLL("../target/debug/libopenai.d")
lib = ctypes.CDLL("libopenai.so")
print("It worked (1)")
print(lib.get_state())
