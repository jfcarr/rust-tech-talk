# Testing

Rust supports [unit testing](https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html), [documentation testing](https://doc.rust-lang.org/stable/rust-by-example/testing/doc_testing.html), and [integration testing](https://doc.rust-lang.org/stable/rust-by-example/testing/integration_testing.html).

## Unit Testing

main.rs
```rust,noplayground
fn main() {
    println!("10 + 5 is {}", add(10, 5));
}

fn add(first_number: i32, second_number: i32) -> i32 {
    return first_number + second_number;
}

#[cfg(test)]
mod tests;
```

tests.rs
```rust,noplayground
use super::*;

#[test]
fn test_add() {
    assert_eq!(add(10, 5), 15);
}
```

Running `cargo test` in the project root directory will invoke all of the test functions, and display the results.
