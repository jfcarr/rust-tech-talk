# Tuple

A tuple provides a means of grouping several values with a variety of types into one compound type.

```rust,editable
fn main() {
	let my_tuple1 = (20, 27.4);
	let my_tuple2: (i32, f64) = (10, 12.22);

	println!("{}", my_tuple1);  // This will fail: The Display trait used by println!
								// doesn't know how to handle tuples.
}
```

Unsupported display types can be handled like this:

```rust,editable
fn main() {
	let my_tuple1 = (20, 27.4);					// Inferred type
	let my_tuple2: (i32, f64) = (10, 12.22);	// Explicit type

	println!("Flexible formatting print: {:?}", my_tuple1);		// Specify ':?' as
																// the format
	println!("Pretty print: {:#?}", my_tuple1);					// Specify ':#?' for
																// pretty-print
}
```

Learn more: [https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type)
