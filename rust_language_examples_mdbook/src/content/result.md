# Result

`Result` provides an enhancement over `Option` in that it describes a possible _error_ instead of a possible _absence_.

```rust,editable
use std::num::ParseIntError;

fn parse_integer(input_value: &str) -> Result<i32, ParseIntError> {
	match input_value.parse::<i32>() {
		Ok(n) => return Ok(n),
		Err(e) => return Err(e),
	};
}

fn main() {
	let result = parse_integer("1");
	println!("{:?}", result);

	let result = parse_integer("text");
	println!("{:?}", result);
}
```