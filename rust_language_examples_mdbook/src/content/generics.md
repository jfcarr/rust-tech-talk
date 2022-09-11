# Generics

Rust provides support for _generics_: abstractions of concrete types.  Generics provide a way to reduce code duplication.

## Duplicate Code

We'll start with a simpler concept: extraction of functions.  Consider the following:

```rust,editable
fn main() {
	println!("Hello!");
	println!("I'm printing a number: 1");

	println!("Hello!");
	println!("I'm printing a string: text");
}
```

## Functions

In the code block above, we have two very similar sets of code lines.  It makes sense to refactor that duplication into a function.  But, in order to provide the flexibility of printing numbers and strings, we have to create _two_ functions:

```rust,editable
fn print_a_number(input_number: i32) {
	println!("Hello!");
	println!("I'm printing a number: {}", input_number);
}

fn print_a_string(input_string: String) {
	println!("Hello!");
	println!("I'm printing a string: {}", input_string);
}

fn main() {
	print_a_number(1);

	print_a_string("text".to_string());
}
```

## A Generic

To accomplish our task in a single function, we can pass the value to printed as a generic representation `<T>`:

> I cheated a little bit in order to keep this example simple: My argument type is being restricted to values that implement `std::fmt::Display`.

```rust,editable
fn print_a_value<T>(input_value: T) where T: std::fmt::Display {
	println!("I'm printing a value: {}", input_value);
}

fn main() {
	print_a_value(1);

	print_a_value("text".to_string());
}
```

----

Learn more: [https://doc.rust-lang.org/stable/book/ch10-00-generics.html](https://doc.rust-lang.org/stable/book/ch10-00-generics.html)
