# Python from Rust

If you want to embed Python into a Rust binary, your Python installation must contain a shared library.  For a Debian-based Linux environment, you install this as follows:

```bash
sudo apt install python3-dev
```

Running Python code from Rust is very straightforward.  First, in an empty directory, create a new Rust project:

```bash
cargo new demo

cd demo
```

I named my project `demo`, but that's just an example.

Add a pyo3 dependency to Cargo.toml:

```properties
[dependencies.pyo3]
version = "0.17.1"
features = ["auto-initialize"]
```

Update `src/main.rs` to call some methods from the Python `sys` library:

```rust,noplayground
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    })
}
```

Compile and run the program:

```bash
cargo run
```

You'll see something like this:

```
Hello jimc, I'm Python 3.10.4 (main, Jun 29 2022, 12:14:53) [GCC 11.2.0]
```

----

Learn more:  [https://pyo3.rs/v0.17.1/#using-python-from-rust](https://pyo3.rs/v0.17.1/#using-python-from-rust)

Many usage examples:  [https://pyo3.rs/v0.17.1/python_from_rust.html](https://pyo3.rs/v0.17.1/python_from_rust.html)