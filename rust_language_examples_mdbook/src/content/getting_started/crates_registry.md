# Crates Registry

The Crates Registry (located at [https://crates.io/](https://crates.io/)) is a catalog of thousands of Rust libraries that you can add to your Rust projects.  It is comparable to the library and package catalogs for other languages:


Language | Catalog
---------|----------
.NET | Nuget
Node.js | npm Registry
Perl | CPAN
Python | PyPI
Ruby | RubyGems

If, for example, you'd like to add the `rand` library to your project, you can execute Cargo in your project directory like this:

```bash
cargo add rand
```

This will add an entry for the library to your project's Cargo.toml file.  You could also add the entry to Cargo.toml manually:

```toml
[dependencies]
rand = "0.8.5"
```
