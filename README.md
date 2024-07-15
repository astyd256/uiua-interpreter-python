# Uiua interpreter for python

This project provides Python bindings for the [Uiua](https://www.uiua.org/) language compiler, allowing you to compile and run uiua code directly from Python. The bindings are created using the pyo3 library.
# Requirements

   * Rust (latest stable version)
   * Python 3.7 or later
   * maturin (for building and developing the bindings)

# Installation
Using maturin

To build and install the bindings using maturin, run the following commands:

Cone the repository:

```sh
git clone https://github.com/yourusername/uiua-compiler-python.git
cd uiua-compiler-python
``` 

Installation using virtual enviroment:

```sh
python -m venv .venv
.venv\Scripts\activate
pip install maturin
maturin build
deactivate
```

Install to your own python:

```

pip install target/wheels/uiua_compiler-[compiler specific].whl
```



# Installing precompiled bindings

You can also try to install prebuild bindings for Windows attached to this repository too:

```sh
pip install uiua_interpreter-0.1.0-cp39-none-win_amd64.whl 
```
# Usage

Here's an example of how to use the uiua-compiler in your Python code:

```python
import uiua_interpreter

# Run uiua code as a string and from file
result = uiua_interpreter.run_code("your uiua code here")
result = uiua_interpreter.run_file("your absolute path to file.ua")

# Print the result
print(result)
```

# License

This project is licensed under the MIT License.

Feel free to customize this template according to the specific details and requirements of your project.