# Traits

A `trait` is a collection of methods defined for an unknown type: `Self`.  They define functionality meant to be shared with multiple types, similar to interfaces and abstract classes.

```rust,editable
trait DataTrait {
	fn new(x: f64, y: f64) -> Self;
	fn product(&self) -> f64;
}

struct Data {
	x: f64,
	y: f64,
}

impl DataTrait for Data {
	fn new(x: f64, y: f64) -> Data {
		Data {x: x, y: y}
	}

	fn product(&self) -> f64 {
		return self.x * self.y;
	}
}

fn main() {
	let mut data: Data = DataTrait::new(2.0, 2.0);

	println!("The value of data.x is {}", data.x);
	println!("The value of data.y is {}", data.y);

	println!("The product of x and y is {}", data.product());
}
```

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/trait.html](https://doc.rust-lang.org/stable/rust-by-example/trait.html)