# Modules

Modules provide a means for splitting code into logical units.  They are similar to namespaces.

Code without modules:

```rust,editable
fn say_hello() {
	println!("Hello!");
}

fn say_goodbye() {
	println!("Goodbye!");
}

fn main() {
	say_hello();
	say_goodbye();
}
```

Code organized as a module:

```rust,editable
mod greetings {
	pub fn say_hello() {
		println!("Hello!");
	}

	pub fn say_goodbye() {
		println!("Goodbye!");
	}
}

fn main() {
	greetings::say_hello();
	greetings::say_goodbye();
}
```

> Functions in a module are private by default.  You use the `pub` keyword to make them accessible.

A use declaration can be used to bind a full function path to a new name.  This is particularly useful for deeply nested modules:

```rust,editable
use greetings::say_hello as hello;

mod greetings {
	pub fn say_hello() {
		println!("Hello!");
	}
}

fn main() {
	hello();
}
```

Modules can be mapped to file hierarchies, allowing you to break up functional units into files.

Assuming you have the following functions in a file named `greetings.rs`:

```rust,noplayground
pub fn say_hello() {
	println!("Hello!");
}

pub fn say_goodbye() {
	println!("Goodbye!");
}
```

You can then access them as follows:

```rust,noplayground
mod greetings;

fn main() {
	greetings::say_hello();
	greetings::say_goodbye();
}
```

Learn more:  [https://doc.rust-lang.org/stable/rust-by-example/mod.html](https://doc.rust-lang.org/stable/rust-by-example/mod.html)