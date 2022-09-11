# Array

Arrays in Rust are similar to tuples, except that all elements must have the same type.  They are fixed length.

```rust,editable
fn main() {
	let my_array1 = [1,2,3,4,5];			// Inferred type and length
	let my_array2: [i32; 5] = [1,2,3,4,5];	// Explicit type and length

	println!("The contents of my_array1 are {:?}", my_array1);
	println!("The contents of my_array2 are {:?}", my_array2);
}
```

Learn more: [https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type)
