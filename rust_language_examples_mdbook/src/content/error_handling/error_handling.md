# Error Handling

Rust provides a variety of ways to deal with error conditions.  At a high level, simplified:


Construct | Typically Used For
---------|----------
`panic` | Unrecoverable errors.
`Option` type | When a value is optional, and the lack of an option may or may not be an error condition.
`Result` | There is a problem, and it should be dealt with.

Learn more: [https://doc.rust-lang.org/stable/rust-by-example/error.html](https://doc.rust-lang.org/stable/rust-by-example/error.html)
