# Borrowing

To access data without taking ownership of it, **borrowing** can be used.

Passing by reference indicates that we want to borrow:

```rust,editable
// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
	println!("This int is: {}", borrowed_i32);
}

fn main() {
	// Create a boxed i32, and a stacked i32
	let boxed_i32 = Box::new(5_i32);
	let stacked_i32 = 6_i32;

	// Borrow the contents of the box. Ownership is not taken,
	// so the contents can be borrowed again.
	println!("Start value: {}", boxed_i32);
	borrow_i32(&boxed_i32);
	println!("Return value: {}", boxed_i32);

	// Same result with a stacked i32.
	println!("Start value: {}", stacked_i32);
	borrow_i32(&stacked_i32);
	println!("Return value: {}", stacked_i32);
}
```
You can't destroy an object while it's borrowed:

```rust,editable
// This function takes ownership of a box and destroys it
fn destroy_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn main() {
    // Create a boxed i32, and a stacked i32
    let boxed_i32 = Box::new(5_i32);

	{
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Can't destroy `boxed_i32` here.  Why?
        // destroy_box_i32(boxed_i32);
        // ^ What happens if you uncomment this line?
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    destroy_box_i32(boxed_i32);
}
```

Details here: [https://doc.rust-lang.org/stable/rust-by-example/scope/borrow.html](https://doc.rust-lang.org/stable/rust-by-example/scope/borrow.html)
