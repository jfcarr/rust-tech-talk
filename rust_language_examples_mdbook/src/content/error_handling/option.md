# Option

Option gives us more control over multiple conditions.

```rust,editable
fn say_hello(name: Option<&str>) {
	match name {
		Some("Jim") => println!("Jim is a great guy!"),
		Some("Todd") => panic!("You can't trust a Todd!"),
		Some(inner) => println!("Hello {}!", inner),
		None => println!("I don't know who you are..."),
	}
}

fn main() {
	let jim = Some("Jim");
	let john = Some("John");
	let todd = Some("Todd");
	let sam = Some("Sam");

	say_hello(jim);
	say_hello(john);
	say_hello(todd);
	say_hello(sam);  // This will never be reached.
}
```

> What happens if we don't cover `None`?