/*
 * lib.rs
 *
 * ref-map - Convenience methods for references of Option and Result.
 * Copyright (c) 2020 Ammon Smith
 *
 * ref-map is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

#![deny(missing_debug_implementations)]
#![warn(missing_docs)]

//! Convenience methods when dealing with references of [`Option`]s and [`Results`].
//!
//! It introduces two traits, `OptionRefMap` and `ResultRefMap`, which add methods
//! to their respective standard library enums to avoid needing to add `.as_ref()`
//! before any `.map()` methods on their value.
//!
//! This is useful when you want to borrow from an `Option` or `Result` but want
//! to avoid the boilerplate of first getting the references to the values contained inside.
//!
//! ```
//! use ref_map::*;
//!
//! let string: Option<String> = Some("hello world\n".into());
//!
//! // Without ref-map:
//! // the .as_ref() is necessary because otherwise it tries to consume the String
//! let message: Option<&str> = string.as_ref().map(|s| s.trim());
//!
//! // With ref-map:
//! let message: Option<&str> = string.ref_map(|s| s.trim());
//! ```
//!
//! Similarly, `.ref_map()` and `.ref_map_err()` are available for `Result`s to mimic
//! their `.map()` and `.map_err()` methods:
//!
//! ```
//! # use ref_map::*;
//! # use std::path::{Path, PathBuf};
//! let answer: Result<PathBuf, String> = Ok(PathBuf::from("/test"));
//!
//! // Mapping borrowed Ok(_) to another type
//! let path: Result<&Path, &String> = answer.ref_map(|p| p.as_path());
//!
//! // Mapping borrower Err(_)
//! let error: Result<&PathBuf, &str> = answer.ref_map_err(|e| e.as_str());
//! ```
//!
//! [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
//! [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html

mod option;
mod result;

#[cfg(test)]
mod test;

pub use self::option::OptionRefMap;
pub use self::result::ResultRefMap;
