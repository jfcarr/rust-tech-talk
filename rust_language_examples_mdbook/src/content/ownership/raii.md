# RAII

Rust enforces **RAII** (_Resource Acquisition Is Initialization_), so whenever an object goes out of scope, its destructor is called and its owned resources are freed.

> Values in Rust are stack allocated by default.  A `Box` is a smart pointer to a heap allocated value of type `T`.

## Unsafe Operation

```rust,editable
fn main() {
    let my_string = bad_reference();
    
	// -> The compiler will evaluate the safety of this operation.
}

fn bad_reference() -> &String {
    let s = String::from("hello");

	println!("{}", my_string);
    
	return &s;	// This tries to return a reference,
				// but the reference is dropped.
}
```

## Safe Operation

```rust,editable
fn main() {
	let my_string = good_value();

	println!("{}", my_string);
    
	// ->	The compiler allows this, because ownership is safely
	//		transferred from good_value() to main()
}

fn good_value() -> String {
    let s = String::from("hello");
    
	return s;
}
```

## Scope Examples

```rust,editable
fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}
```

Details here: [https://doc.rust-lang.org/stable/rust-by-example/scope/raii.html](https://doc.rust-lang.org/stable/rust-by-example/scope/raii.html).
