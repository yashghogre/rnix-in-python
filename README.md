### rnix in Python

To Build:

First create a virtual environment in the home directory and install maturin, then follow the below steps:

```
cd rnix_python

cargo clean

maturin develop --release

cd ../py_project

python main.py
```
