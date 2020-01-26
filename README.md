## ref-map

[![Build Status](https://travis-ci.org/ammongit/rust-ref-map.svg?branch=master)](https://travis-ci.org/ammongit/rust-ref-map)

Rust crate for a convenience trait on `Option<T>` and `Result<T, E>`.

Has no dependencies, and should work on any Rust release channel.

This crate contains two traits which can be imported to add the `ref_map` methods:

```rust
use ref_map::*;

let string: Option<String> = Some("hello world\n".into());

// Without ref-map:
// the .as_ref() is necessary because otherwise it tries to consume the String
let message: Option<&str> = string.as_ref().map(|s| s.trim());

// With ref-map:
let message: Option<&str> = string.ref_map(|s| s.strim());
```

`ref_map()` is also provided for `Result<T, E>` for `Ok`, and `ref_map_err()` for `Err`.

----

Copyright (C) 2020 Ammon Smith

Available under the MIT License.
