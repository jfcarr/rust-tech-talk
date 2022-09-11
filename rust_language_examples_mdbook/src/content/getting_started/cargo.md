# Cargo

Cargo is the Rust build tool and package manager.  It allows you to do things like this:


Command | Description
---------|----------
`cargo new` | Create a new Rust project
`cargo build` | Build a project
`cargo run` | Run a project
`cargo test` | Execute unit tests in a project
`cargo doc` | Build documentation for a project
`cargo publish` | Publish a project to `crates.io`

For example, to create new Rust binary project, run the following:

```bash
cargo new [project_name]
```

This will scaffold a new project with the following structure:

```
[project_name]
.
├── Cargo.toml
└── src
    └── main.rs
```

Learn more: [https://doc.rust-lang.org/stable/cargo/](https://doc.rust-lang.org/stable/cargo/)
