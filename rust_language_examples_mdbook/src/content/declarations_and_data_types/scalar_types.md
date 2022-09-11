# Scalar Types

The scalar types in Rust are integers, floating point, chars, and booleans.

Learn more: [https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types)

## Integer

Integer types:

Length | Signed | Unsigned
-------|--------|---------
8-bit | i8 | u8
16-bit | i16 | u16
32-bit | i32 | u32
64-bit | i64 | u64
128-bit | i128 | u128
arch | isize | usize

Integer literals:

```rust,editable
fn main() {
	let decimal_value:i32 = 98_222;
	println!("Decimal 98_222 is integer {}", decimal_value);

	let hex_value: i32 = 0xff;
	println!("Hex 0xff is integer {}", hex_value);

	let octal_value: i32 = 0o77;
	println!("Octal 0o77 is integer {}", octal_value);

	let binary_value: i32 = 0b0111;
	println!("Binary 0b0111 is integer {}", binary_value);

	let byte_value: u8 = b'A';
	println!("Byte b'A' is integer {}", byte_value);
}
```

## Floating Point

Rust supports two floating point types: `f32` (32 bits) and `f64` (64 bits).

```rust,editable
fn main() {
	let float_var1: f32 = 10.0;

	let float_var2 = 20.0;  // defaults to f64
}
```

## Character

The char type is Rust's most primitive alphabetic type.

```rust,editable
fn main() {
	let my_char = 'a';

	println!("The value of my_char is {}", my_char);
}
```

In Rust, char type assignments are enclosed in single quotes, and strings are enclosed in double quotes:

```rust,editable
fn main() {
	let mut my_char = 'a';
	let my_string = "a";

	my_char = "b";  // This will fail!
}
```

## Boolean

Boolean types in Rust have two possible values: `true` and `false`.  They are one byte in size.

```rust,editable
fn main() {
	let my_bool1 = true;			// inferred type
	let my_bool2: bool = false;		// explicit type annotation
}
```
