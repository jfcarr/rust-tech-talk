# Crates

A crate is a compilation unit.  Crates can be compiled as binaries or libraries.

In an empty directory, create two subdirectories: `lib` and `bin`.  Then, in the `lib` directory, create a library crate by creating a file named `greeting.rs`, with the following contents:

```rust,noplayground
pub fn say_hello() {
	say_greeting("Hello".to_string());
}

pub fn say_goodbye() {
	say_greeting("Goodbye".to_string());
}

fn say_greeting(prefix: String) {
	println!("{}!", prefix);
}
```

Build the library with the following command:

```bash
rustc --crate-type=lib greeting.rs
```

This will create a file named `libgreeting.rlib`.

Next, in the bin directory, create a file named `client.rs`, with the following contents:

```rust,noplayground
fn main() {
    greeting::say_hello();

    greeting::say_goodbye();
}
```

Build the client (and link it to the library) with this:

```bash
rustc client.rs --extern greeting=../lib/libgreeting.rlib --edition=2018
```

This will create an executable named `client`.  When you run it, you'll see this:

```
Hello!
Goodbye!
```
