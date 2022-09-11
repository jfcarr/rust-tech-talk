# Standard Library Types

The Rust `std` library expands on primitives with a large number of custom types.

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/std.html](https://doc.rust-lang.org/stable/rust-by-example/std.html)

## Strings

Rust supports two string types: `String` and `&str`.  A `String` is a vector of bytes (`Vec<u8>`), and `&str` is a slice (`&[u8]`) pointing to a UTF-8 sequence.

```rust,editable
fn main() {
	// Create a reference to a string in read-only memory:
	let string_slice: &'static str = "This is a string reference";
	println!("The value of string_slice is {}", string_slice);

	let mut vector_string: String = "This is a string vector".to_string();
	println!("The value of vector_string is {}", vector_string);
}
```

> **[Exercise]** What happens if you try to assign a new value to string_slice?

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/std/str.html](https://doc.rust-lang.org/stable/rust-by-example/std/str.html)