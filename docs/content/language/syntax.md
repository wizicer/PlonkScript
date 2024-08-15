Built on the Rhai programming language, Plonk Script adopts a syntax similar to Rust but without explicit typing, thereby reducing code redundancy.

Here some syntax example is taken from [rhai](https://rhai.rs/book/ref/index.html) docs.


## Comments

Comments are C-style, including `/*` … `*/` pairs for block comments and `//` for comments to the end of the line.

Block comments can be nested.

```rust
let /* intruder comment */ name = "Bob";

// This is a very important one-line comment

/* This comment spans
   multiple lines, so it
   only makes sense that
   it is even more important */
```