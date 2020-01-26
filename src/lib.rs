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

//! TODO

mod option;
mod result;


pub use self::option::OptionRefMap;
pub use self::result::ResultRefMap;
