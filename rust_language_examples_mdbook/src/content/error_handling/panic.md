# panic

```rust,editable
fn say_hello(name: String) {
	if name == "Todd" {
		panic!("You can't trust a Todd!");
	} else {
		println!("Hello, {}!", name);
	}
}

fn main() {
	say_hello("Jim".to_string());
	
	say_hello("Todd".to_string());

	say_hello("Sam".to_string()); // This will never be reached.
}
```

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/error/panic.html](https://doc.rust-lang.org/stable/rust-by-example/error/panic.html)
