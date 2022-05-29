use std::io as stdio;
use rustsdk::io;
use std::path::{Path};

pub fn read_file<P: AsRef<Path>>(p: P)->Result<Vec<u8>,stdio::Error>{
    return io::read_file(p);
}
