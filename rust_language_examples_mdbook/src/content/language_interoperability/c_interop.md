# C Interop

C code interoperability is accomplished via the Foreign Function Interface.

Let's start with a simple C source file, along with its header file:

simplemath.c
```c
#include "simplemath.h"

float square(float input) { return input * input; }
```

simplemath.h
```c
float square(float input);
```

To use this from Rust, we need to first compile the C source, to create `simplemath.o` (the object file):

```bash
gcc -c simplemath.c
```

Then, create an .ar archive file (`libsimplemath.a`) from `simplemath.o`:

```bash
ar rcs libsimplemath.a simplemath.o
```

Next, we'll create a Rust file that will call the C library:

main.rs
```rust,noplayground
/* This 'extern' block describes the C library function we want to call. */
extern "C" {
    fn square(x: f32) -> f32;
}

fn main() {
	/* Our input to the external function */
    let input_value: f32 = 10.0;

	/* Since we're calling an external function, it must be marked as unsafe */
    unsafe {
        println!("The square of {} is {}", input_value, square(input_value));
    }
}
```

Finally, we compile our Rust crate, and link the external library:

```bash
rustc -l static=simplemath -L . main.rs
```

This produces a binary named `main`, and when we run it, we get this result:

```
./main

The square of 10 is 100
```
