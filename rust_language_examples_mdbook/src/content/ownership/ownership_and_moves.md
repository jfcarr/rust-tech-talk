# Ownership and Moves

Resources can have only one owner.  When ownership is transferred to a new owner, the previous owner can no longer be used.

## Simple stack-allocated types

```rust,editable
fn main() {
	// 'x' is a stack allocated integer.
	let x = 5u32;

	// Assignment of a simple type like 'x' implements a copy, so the data is
	// copied to 'y' and no resources are moved.
	let y = x;

	// Both values can be independently used
	println!("x is {}, and y is {}", x, y);
}
```

## Complex heap-allocated types

```rust,editable
fn main() {
	// 'a' is a pointer to a heap allocated integer
	let a = Box::new(5i32);

	println!("a contains: {}", a);

	// 'b' takes ownership
	let b = a;

	println!("b contains: {}", b);

	// println!("a contains {}", a);  // What happens if you uncomment this line?
}
```

```rust,editable
// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
	println!("Destroying a box that contains {}", c);

	// 'c' is destroyed and the memory freed
}

fn main() {
	let a = Box::new(5i32);

	println!("a contains: {}", a);

	destroy_box(a);

	// println!("a contains: {}", a);  // What happens if you uncomment this line?
}
```

Details here: [https://doc.rust-lang.org/stable/rust-by-example/scope/move.html](https://doc.rust-lang.org/stable/rust-by-example/scope/move.html)
