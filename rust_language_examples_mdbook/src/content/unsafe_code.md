# Unsafe Code

The default behavior of Rust is to provide as many safety checks-and-balances as possible, to protect you from unsafe code.  But, sometimes it's necessary to bypass these protections, for things like:

* dereferencing raw pointers
* calling functions over FFI
* accessing hardware addresses directly, for IOT/embedded programming

To write unsafe code, you simply wrap the code in an `unsafe` block, like this:

```rust,editable
fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }
}
```

Learn more:  [https://doc.rust-lang.org/stable/rust-by-example/unsafe.html](https://doc.rust-lang.org/stable/rust-by-example/unsafe.html)
