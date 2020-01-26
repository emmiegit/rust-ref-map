/*
 * option.rs
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

/// Adds the `ref_map()` extension method onto [`Option`].
///
/// The `ref_map()` method borrows the internal object and passes it
/// to a closure to be mapped in some way. This allows convenient use
/// of `as_*` type methods or calculations which require the borrowed
/// internal value.
///
/// ```
/// use ref_map::*;
///
/// let values = Some(vec![4, 7, 9, 5, 6]);
///
/// let filtered = values.ref_map(|v| &v[3..]);
/// let answer = &[5, 6];
///
/// assert_eq!(filtered, Some(&answer[..]));
/// ```
///
/// See the crate-level documentation for more information.
///
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
pub trait OptionRefMap<'o, T: 'o> {
    /// Borrows the internal value and maps it using the provided closure.
    fn ref_map<U, F>(&'o self, f: F) -> Option<U>
    where
        F: FnOnce(&'o T) -> U;
}

impl<'o, T: 'o> OptionRefMap<'o, T> for Option<T> {
    #[inline]
    fn ref_map<U, F>(&'o self, f: F) -> Option<U>
    where
        F: FnOnce(&'o T) -> U,
    {
        match *self {
            Some(ref x) => Some(f(x)),
            None => None,
        }
    }
}
