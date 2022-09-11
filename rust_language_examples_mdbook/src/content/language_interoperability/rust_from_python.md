# Rust from Python

To create our Rust-based Python module, we'll be making use of `maturin`, a build/publish tool.

First, we'll create a directory for our project, and `cd` to it:

```bash
mkdir string_sum

cd string_sum
```

Next, create a Python virtual environment, and activate it:

```bash
python3 -m venv .env

source .env/bin/activate
```

Install `maturin` into the Python environment:

```bash
pip3 install maturin
```

Scaffold your project:

```bash
maturin init
```

Select `pyo3` for your bindings:

```
? 🤷 Which kind of bindings to use? ›
❯ pyo3
  rust-cpython
  cffi
  bin
```

A starter project will be generated, with sample code for `Cargo.toml` and `src/lib.rs`.  The code provides a simple function accepting two numbers as input, adding them together, and returns the result as a string:

```rust,noplayground
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
```

Finally, build your package and install it into the virtual environment:

```bash
maturin develop
```

Now we're ready to test our new Rust-based package from Python.  Start a Python interactive environment:

```bash
python3
```

Then, import the new package and run the function:

```
>>> import string_sum
>>> string_sum.sum_as_string(1,1)
2
```

Learn more: [https://pyo3.rs/v0.17.1/#using-rust-from-python](https://pyo3.rs/v0.17.1/#using-rust-from-python)
