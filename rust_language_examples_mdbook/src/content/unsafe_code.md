# Unsafe Code

The default behavior of Rust is to provide as many safety checks-and-balances as possible, to protect you from unsafe code.  For example, if you try to derefence a raw pointer, the Rust compiler disallows it:

```rust,editable
fn main() {
    let raw_p: *const u32 = &10;

	assert!(*raw_p == 10);
}
```

But, sometimes it's necessary to bypass these protections, not only for raw pointers, but also for operations like calling functions over FFI, and accessing hardware addresses directly (for IoT/embedded programming).

To write unsafe code, you simply wrap the code in an `unsafe` block, like this:

```rust,editable
fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}
```

An unsafe wrapper informs the Rust compiler that you know what you're doing, and to turn off its safety checks.

Learn more:

* [https://doc.rust-lang.org/stable/rust-by-example/unsafe.html](https://doc.rust-lang.org/stable/rust-by-example/unsafe.html)
* [https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html](https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html)
