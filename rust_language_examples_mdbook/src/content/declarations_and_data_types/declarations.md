# Declarations

## Statically Typed

Rust is statically typed.  Variables must be declared.  If a type is not specified, Rust will try to infer it.

```rust,editable
fn main() {
	let my_variable1: i32 = 5;  // Explicit i32
	let my_variable2 = 5;		// Inferred i32

	println!("The value of my_variable1 is {}", my_variable1);
	println!("The value of my_variable2 is {}", my_variable2);
}
```

## Type Conversion

There is no implicit type conversion in Rust:

```rust,editable
fn main() {
	// This won't work, because the 5.0 will be treated as a float:
	let my_variable: i32 = 5.0;

	println!("The value of my_variable is {}", my_variable);
}
```

But, you can use casting for explicit conversion:

```rust,editable
fn main() {
	let my_variable: i32 = 5.0 as i32;

	println!("The value of my_variable is {}", my_variable);
}
```

## Mutability

Variables are immutable by default:

```rust,editable
fn main() {
	let my_variable: i32 = 5;

	println!("The value of my_variable is {}", my_variable);

	let my_variable = 10;	// This works, because 'let' creates a new instance of
							// the variable.

	println!("The value of my_variable is {}", my_variable);

	my_variable = 15;  	// This does NOT work, because it attempts to assign a value
						// to the existing instance of the immutable variable.

	println!("The value of my_variable is {}", my_variable);
}
```

If you want variables to be mutable, use the `mut` keyword:

```rust,editable
fn main() {
	let mut my_variable: i32 = 5;

	println!("The value of my_variable is {}", my_variable);

	my_variable = 10;

	println!("The value of my_variable is {}", my_variable);
}
```