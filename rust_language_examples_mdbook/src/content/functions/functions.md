# Functions

Functions are declared with the `fn` keyword.  Arguments are type-annotated.  If the function returns a value, it must be specified after an arrow.

> The order of function definitions are not dependent on `main()`.

Let's start with some code that needs refactoring:

```rust,editable
fn main() {
	println!("Hello, Jim!");
	println!("Hello, John!");

	let square_of_two:i32 = 2 * 2;
	let square_of_four:i32 = 4 * 4;

	println!("2 squared is {}", square_of_two);
	println!("4 squared is {}", square_of_four);
}
```

We have two instances of code duplication that can be refactored into functions:

1. The "Hello" greeting, and
2. The calculation of squares.

Let's start with "Hello":

```rust,editable
fn say_hello(name: String) {
	println!("Hello, {}!", name);
}

fn main() {
	say_hello("Jim".to_string());
	say_hello("John".to_string());

	let square_of_two:i32 = 2 * 2;
	let square_of_four:i32 = 4 * 4;

	println!("2 squared is {}", square_of_two);
	println!("4 squared is {}", square_of_four);
}
```

Next, we'll add a function to handle calculating square values.  We need a return value for this.

```rust,editable
fn say_hello(name: String) {
	println!("Hello, {}!", name);
}

fn squared(input: i32) -> i32 {
	return input * input;
}

fn main() {
	say_hello("Jim".to_string());
	say_hello("John".to_string());

	let square_of_two:i32 = 2 * 2;
	let square_of_four:i32 = 4 * 4;

	println!("2 squared is {}", squared(2));
	println!("4 squared is {}", squared(4));
}
```

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/fn.html](https://doc.rust-lang.org/stable/rust-by-example/fn.html)