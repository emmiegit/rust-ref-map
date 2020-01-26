/*
 * result.rs
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

pub trait ResultRefMap<'t, 'e, T: 't, E: 'e> {
    fn ref_map<U, F>(&self, f: F) -> Result<U, &E>
    where
        F: FnOnce(&T) -> U,
        U: 't;

    fn ref_map_err<D, F>(&self, f: F) -> Result<&T, D>
    where
        F: FnOnce(&E) -> D,
        D: 'e;
}

impl<'t, 'e, T: 't, E: 'e> ResultRefMap<'t, 'e, T, E> for Result<T, E> {
    #[inline]
    fn ref_map<U, F>(&self, f: F) -> Result<U, &E>
    where
        F: FnOnce(&T) -> U,
        U: 't,
    {
        match *self {
            Ok(ref x) => Ok(f(x)),
            Err(ref x) => Err(x),
        }
    }

    #[inline]
    fn ref_map_err<D, F>(&self, f: F) -> Result<&T, D>
    where
        F: FnOnce(&E) -> D,
        D: 'e,
    {
        match *self {
            Ok(ref x) => Ok(x),
            Err(ref x) => Err(f(x)),
        }
    }
}
