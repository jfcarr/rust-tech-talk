# Standard Library - Misc

The standard library provides additional complex types to support things like:

* Threads
* Channels
* File I/O
* Program Arguments
* Foreign Function Interface

> You can find enhanced alternatives for many of these by visiting [crates.io](https://crates.io/).  For example, `clap` is a popular crate for working with program arguments.

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/std_misc.html](https://doc.rust-lang.org/stable/rust-by-example/std_misc.html)

## File I/O - Open

```rust,noplayground
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
```

(Code example is copied directly from [here](https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/open.html).)
