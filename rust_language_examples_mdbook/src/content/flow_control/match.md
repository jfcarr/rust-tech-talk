# match

The `match` keyword provides pattern matching similar to a C `switch`.

```rust,editable
fn main() {
	let number:i32 = 10;

	match number {
		1 => println!("The value is one!"),
		2 | 3 | 4 =>
		 println!("The value is two, three, or four (actual value: {})", number),
		5..=8 =>
		 println!("The value is five, six, seven, or eight (actual value: {})", number),
		_ => println!("No special handling for {}", number),
	}
}
```

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/flow_control/match.html](https://doc.rust-lang.org/stable/rust-by-example/flow_control/match.html)