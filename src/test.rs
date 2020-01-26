/*
 * test.rs
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

use crate::*;
use std::path::{Path, PathBuf};

#[test]
fn test_option() {
    let path: Option<PathBuf> = Some(PathBuf::from("/test"));

    let a: Option<&Path> = path.as_ref().map(|p| p.as_path());
    let b: Option<&Path> = path.ref_map(|p| p.as_path());

    assert_eq!(a, b);
}

#[test]
fn test_ok() {
    let path: Result<PathBuf, ()> = Ok(PathBuf::from("/test"));

    let a: Result<&Path, &()> = path.as_ref().map(|p| p.as_path());
    let b: Result<&Path, &()> = path.ref_map(|p| p.as_path());

    assert_eq!(a, b);
}

#[test]
fn test_err() {
    let string: Result<(), String> = Err(String::from("apple banana "));

    let a: Result<&(), &str> = string.as_ref().map_err(|s| s.trim());
    let b: Result<&(), &str> = string.ref_map_err(|s| s.trim());

    assert_eq!(a, b);
}
