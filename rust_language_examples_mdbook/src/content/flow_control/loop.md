# loop

The `loop` keyword provides an infinite loop.

```rust,editable
fn main() {
	let mut count:i32 = 0;

	loop {
		count += 1;

		if count == 5 {
			break;
		}

		if count == 3 {
			continue;
		}

		println!("Iteration: {}", count);
	}
}
```

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/flow_control/loop.html](https://doc.rust-lang.org/stable/rust-by-example/flow_control/loop.html)
