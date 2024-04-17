# The Aim of this repo
The aim is to provide an already set up environment using rust and python to code. But actually at the moment it is just a repo for my own purposes, so I don't care if there is bad coding and it is not suitable for other use-cases. **Also note that i'm on linux, so idk if there may be some problem with that if you are on other platform**. 

# What can you do
- Rust coding in src/lib.rs file
  - Write pymodule in which you can add pyfunction, pyclasses, etc...
    - For data type conversion from Rust type to Python type see [PyO3 data type table conversion](https://pyo3.rs/main/conversions/tables.html)
- Run the Rust code you've written calling them in a Python file

# In order to start
- Just:
  1. `git clone` the repo
  2. Activate the virtualenv
  3. Run `maturin develop && python3 main.py` or `maturin develop --release && python3 main.py` (linux commands) to compile Rust code with optimizations.

# Useful references
- [Pyo3 user's guide](https://pyo3.rs/v0.21.2/)
- [Data type conversion Rust-Python](https://pyo3.rs/main/conversions/tables.html)
- [PyO3 Rust documentation](https://docs.rs/pyo3/latest/pyo3/)

# Python type hinting on rust imported function and classes
  After the rust code is compiled with `maturin develop`, the rust library will be available to import in a python program (using the virtual environment python interpreter) but type hinting does not come automatically, so you'll need to write the .pyi file and define all the type hinting for your rust functions, classes, etc... . The name of the .pyi name should be the function's name that is tagged with #[pymodule], in lib.rs.
  (So just write the .pyi file with python sintax type hinting and add '''docs comment''' as the body of the function)
 
# Tools used
This is just an already initialialized [maturin](https://github.com/PyO3/maturin) project with [PyO3](https://github.com/PyO3/pyo3).


*you can follow maturin guides, this repo is just ready to use*

# Dependencies
- Rust: see [here](https://www.rust-lang.org/tools/install) for install instructions
- virtualenv python module (pip available)
- maturin (pip available)
- patchelf (pip available)
