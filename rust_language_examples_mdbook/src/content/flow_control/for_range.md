# for and range

The `for in` construct can be used to iterate through an `Iterator`.  For a simple `Iterator`, a `range` can be used.

> The upper value of the range is not inclusive.

```rust,editable
fn main() {
	for item in 1..5 {
		println!("Iteration: {}", item);
	}
}
```

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/flow_control/for.html](https://doc.rust-lang.org/stable/rust-by-example/flow_control/for.html)