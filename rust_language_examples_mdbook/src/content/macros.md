# Macros

You can use Rust's macro system to implement [metaprogramming](https://codedocs.org/what-is/metaprogramming).

A macro looks and acts very much like a function, except that a macro name ends with a bang !, and instead of generating a function call, macros are expanded into source code that is compiled along with the rest of the program.

Here's simple greeting functionality implemented as a function:

```rust,editable
fn say_hello(name: String) {
	println!("Hello, {}!", name);
}

fn main() {
	say_hello("John".to_string());
}
```

And here's the same functionality implemented as a macro:

```rust,editable
macro_rules! say_hello {
	($name:expr) => {
		println!("Hello, {}!", $name);
	};
}

fn main() {
	say_hello!("John".to_string());
}
```

The $name expression in the example above is one of many kinds of `designators` supported by Rust macros.  This provides a level of abstraction somewhat similar to `generics`, with more flexibility than concrete functions.

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/macros.html](https://doc.rust-lang.org/stable/rust-by-example/macros.html)
