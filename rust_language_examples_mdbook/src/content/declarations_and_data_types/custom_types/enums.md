# Enums

There are a few variants of the enum type.  Here's the simplest:

```rust,editable
fn main() {
	enum EventStatus {
		OK,
		Failed
	}

	let my_result = EventStatus::OK;

	match my_result {
		EventStatus::OK     => println!("Finished successfully!"),
		EventStatus::Failed => println!("Failed..."),
	}
}
```

Learn more:  [https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum.html](https://doc.rust-lang.org/stable/rust-by-example/custom_types/enum.html)
