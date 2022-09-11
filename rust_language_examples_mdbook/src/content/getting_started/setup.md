# Setup

The best way to set up and maintain a Rust installation is to use the `rustup` tool.

If you're running Linux, it's a one-line command.  Run this, and follow the prompts:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you are installing in Windows, you may also need to install the Visual Studio Build Tools.  The `rustup` CLI will prompt you if this is the case.

> After you've installed Rust, you can update to the latest version at any time by running `rustup update`.

After rustup finishes, test your installation by running `rustc --version`.  You should see something like this:

```
rustc 1.63.0 (4b91a6ea7 2022-08-08)
```

If you'd like to actually test the compiler, create a file named `main.rs`, with the following contents:

```rust,noplayground
fn main() {
	println!("Hello, world!");
}
```

Then, run this:

```bash
rustc main.rs
```

This will produce a binary file called `main`.  When you run it, you should see this:

```
Hello, world!
```

Learn more:  [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started)
