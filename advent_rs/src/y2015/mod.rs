#![allow(dead_code)]

mod d7;
mod d8;
mod d9;
mod d10;
mod d11;
mod d12;

use std::fs::File;
use std::io::Read;
use std::io::Write;

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("read error");
    contents
}

fn write_file(filename: &str, text: &str) {
    let mut file = File::create(filename).expect("cannot create file");
    write!(file, "{}", text).expect("write error");
}
