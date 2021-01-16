/*
 * result.rs
 *
 * ref-map - Convenience methods for references of Option and Result.
 * Copyright (c) 2020-2021 Ammon Smith
 *
 * ref-map is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

/// Adds the `ref_map()` and `ref_map_err()` extension methods onto [`Result`].
///
/// The `ref_map()` method borrows the internal object and passes it
/// to a closure to be mapped in some way. This allows convenient use
/// of `as_*` type methods or calculations which require the borrowed
/// internal value.
///
/// ```
/// # #[derive(Debug, PartialEq, Eq)]
/// # struct Error;
/// use ref_map::*;
///
/// let answer: Result<Vec<i8>, Error> = Ok(vec![4, 7, 9, 5, 6]);
///
/// let filtered = answer.ref_map(|v| &v[3..]);
/// let answer = &[5, 6];
///
/// assert_eq!(filtered, Ok(&answer[..]));
/// ```
///
/// The `ref_map_err()` provides the same, but on the `Err(_)` variant
/// instead of `Ok(_)`.
///
/// ```
/// # use ref_map::*;
///
/// let answer: Result<(), String> = Err("file not found".into());
///
/// let error = answer.ref_map_err(|s| s.as_str());
///
/// assert_eq!(error, Err("file not found"));
/// ```
///
/// See the crate-level documentation for more information.
///
/// [`Result`]: https://doc.rust-lang.org/std/option/enum.Result.html
pub trait ResultRefMap<'r, T: 'r, E: 'r> {
    /// Borrows the value in the `Ok` variant and maps it using the provided closure.
    fn ref_map<U, F>(&'r self, f: F) -> Result<U, &E>
    where
        F: FnOnce(&'r T) -> U + 'r;

    /// Borrows the value in the `Err` variant and maps it using the provided closure.
    fn ref_map_err<D, F>(&'r self, f: F) -> Result<&T, D>
    where
        F: FnOnce(&'r E) -> D + 'r;
}

impl<'r, T: 'r, E: 'r> ResultRefMap<'r, T, E> for Result<T, E> {
    #[inline]
    fn ref_map<U, F>(&'r self, f: F) -> Result<U, &E>
    where
        F: FnOnce(&'r T) -> U + 'r,
    {
        match *self {
            Ok(ref x) => Ok(f(x)),
            Err(ref x) => Err(x),
        }
    }

    #[inline]
    fn ref_map_err<D, F>(&'r self, f: F) -> Result<&T, D>
    where
        F: FnOnce(&'r E) -> D + 'r,
    {
        match *self {
            Ok(ref x) => Ok(x),
            Err(ref x) => Err(f(x)),
        }
    }
}
