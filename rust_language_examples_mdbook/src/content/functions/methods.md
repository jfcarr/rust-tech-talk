# Methods

Functions can be connected to a type.  They can be defined on the type ("associated functions") or called on an instance of a type ("methods").

Associated functions are generally used like constructors:

```rust,editable
struct Data {
	x: f64,
	y: f64,
}

impl Data {
	fn new(x: f64, y: f64) -> Data {
		Data {x: x, y: y}
	}
}

fn main() {
	let data = Data::new(2.0, 2.0);

	println!("The value of data.x is {}", data.x);
	println!("The value of data.y is {}", data.y);
}
```

Methods operate on an instance of a type:

```rust,editable
struct Data {
	x: f64,
	y: f64,
}

impl Data {
	fn new(x: f64, y: f64) -> Data {
		Data {x: x, y: y}
	}

	// Methods use the built-in argument &self, which provides a reference to
	// an instance of the type.
	fn product(&self) -> f64 {
		return self.x * self.y;
	}
}

fn main() {
	let data = Data::new(2.0, 2.0);

	println!("The value of data.x is {}", data.x);
	println!("The value of data.y is {}", data.y);

	println!("The product of x and y is {}", data.product());
}
```

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/fn/methods.html](https://doc.rust-lang.org/stable/rust-by-example/fn/methods.html)
