# The Aim of this repo
The aim is to provide an already set up environment using rust and python to code.

# What can you do
- Rust coding in lib.rs
  - Write pymodule in which you can add pyfunction, pyclasses, etc...
    - For data type conversion from Rust type to Python type see [PyO3 data type table conversion](https://pyo3.rs/main/conversions/tables.html)
- Run the Rust code you've written calling them in Python


# Python type hinting on rust imported function and classes
  After the rust code is compiled with `maturin develop`, the rust library will be available to import in a python program (using the virtual environment python interpreter) but type hinting does not come automatically, so you'll need to write the rust_lib_name.pyi file and define all the type hinting for your rust functions, classes, etc... .
  (So just write the .pyi file)
 
# Tools used
This is just an already initialialized [maturin](https://github.com/PyO3/maturin) project with [PyO3](https://github.com/PyO3/pyo3)
