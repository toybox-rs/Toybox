# Build instructions

`toybox/toybox/toybox.py` is hard-coded to look up the compiled rust library for OSX. This is because different users' security settings may prevent using DYLD_LIBRARY_PATH on OSX. The equivalent environment variable for Linux is LD_LIBRARY_PATH. See https://github.com/jjfiv/toybox/blob/e62014ce067e598c5e0dd4819f2c78a9fc2ff027/openai/toybox/toybox/toybox.py#L7.


When running on Linux, you should be able to remove the path part of the Python code that loads in. The executable will have a different name, i.e., `libopenai.so`. When you run the Python code that uses this library, you can set LD_LIBRARY_PATH locally:
`LD_LIBRARY_PATH=<path_to_folder_containing_libopenai.so> python some_program_in_python.py`


Finally, compile the R code using `cargo build --release`.
