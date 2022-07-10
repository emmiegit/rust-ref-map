## ref-map

[![Build Status](https://travis-ci.org/emmiegit/rust-ref-map.svg?branch=master)](https://travis-ci.org/emmiegit/rust-ref-map)

Rust crate for convenience traits on `Option<T>` and `Result<T, E>`.

Has no dependencies, and should work on any Rust release channel.

Three methods are provided, `ref_map()` for `Some(_)` and `Ok(_)`, and `ref_map_err()` for `Err(_)`.
This allows easily mapping borrowed values from maybe values.

```rust
use ref_map::*;

let string: Option<String> = Some("hello world\n".into());

// Without ref-map:
// the .as_ref() is necessary because otherwise it tries to consume the String
let message: Option<&str> = string.as_ref().map(|s| s.trim());

// With ref-map:
let message: Option<&str> = string.ref_map(|s| s.trim());
```

`ref_map()` is also provided for `Result<T, E>` for `Ok`, and `ref_map_err()` for `Err`.

----

Copyright (C) 2020-2021 Ammon Smith

Available under the MIT License.
