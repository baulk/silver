// pack.rs
use git2::Repository;
use std::fs::File;
use std::io::prelude::*;

struct Blob<T> {
    offset: T,
    index: i32,
}

// https://doc.rust-lang.org/std/io/struct.BufReader.html

/// Repository large analyze executor
struct Executor {
    limit: i64,
    hashlen: i64,
    path: str,
}

impl Executor {
    fn create(repoDir: str, limit: i64) -> Result<Executor, &'static str> {
        Err("unimpl")
    }
}
