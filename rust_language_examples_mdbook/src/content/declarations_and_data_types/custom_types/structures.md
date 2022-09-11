# Structures

Rust supports three types of structures:

* Named tuples
* Classic C structs
* Unit structs (useful for generics)

Tuple example:

```rust,editable
fn main() {
	struct TupleExample(i32, f32);

	// Instanciate:
	let tuple_example = TupleExample(1, 2.2);

	// Access the fields by position:
	println!("tuple_example contains {:?} and {:?}", tuple_example.0, tuple_example.1);

    // Destructure:
	let TupleExample(integer_field, decimal_field) = tuple_example;
    
	println!("tuple_example contains {:?} and {:?}", integer_field, decimal_field);
}
```

Classic example:

```rust,editable
fn main() {
	struct ClassicExample {
		string_field: String,
		integer_field: u8,
	}

	let classic_example: ClassicExample = 
	 ClassicExample { string_field: String::from("A String value"), integer_field: 10 };

	println!("classic_example values: ({}, {})",
	 classic_example.string_field, classic_example.integer_field);
}
```

Unit example:

```rust,editable
fn main() {
	struct UnitExample;

	let _unit_example = UnitExample;
}
```

Unit structs have no fields, and are useful in generics.

> Note the use of the underscore in `_unit_example`, indicating to the compiler that the variable is intentionally unused.

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html](https://doc.rust-lang.org/stable/rust-by-example/custom_types/structs.html)
