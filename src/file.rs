#![allow(unused)]
use std::fs::{self, File};
use std::io::prelude::*;

pub fn run() -> std::io::Result<()> {
    let mut file = fs::File::create("foo.txt")?;
    file.write_all(b"Hello, world!!!")?;
    Ok(())
}

// fs::remove_file("foo.txt")?; - remove-file

pub fn open() -> std::io::Result<()> {
    let mut file = File::open("foo.txt")?;
    let file_copy = file.try_clone()?;
    Ok(())
}
