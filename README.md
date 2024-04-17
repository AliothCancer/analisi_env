# The Aim of this repo
The aim is to provide an already set up environment using rust and python to code.


# Workflow
I thought it in this way:
- Write library code in rust
  - Using it for heavy lifting
- Write python:
  - To visualize data
  - Note: Or make some rust function to plot using some rust library
- Use a custom script to compile the library and run the main.py program

# Python type hinting on rust imported function and classes
  After the rust code is compiled with `maturin develop` library will be available to import in a python program (using the virtual environment python interpreter) but type hinting does not come automatically, so you'll need to write the rust_lib_name.pyi file and define all the type hinting for your rust functions, classes, etc... 
 
# Tools used
This is just an already initialialized [maturin](https://github.com/PyO3/maturin) project with [PyO3](https://github.com/PyO3/pyo3)
