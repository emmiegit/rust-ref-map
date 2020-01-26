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

pub trait OptionRefMap<T> {
    fn ref_map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Option<U>;
}

impl<T> OptionRefMap<T> for Option<T> {
    #[inline]
    fn ref_map<U, F: FnOnce(&T) -> U>(&self, f: F) -> Option<U> {
        match *self {
            Some(ref x) => Some(f(x)),
            None => None,
        }
    }
}
