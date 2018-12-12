use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn load_input() -> std::io::Result<Vec> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let mut v<str> = vec![];
    for res in buf_reader.lines() {
        let s = res?;
        let trimmered = s.trim();
        v.push_back(s);
    }
    v;
}

fn main() {
    let v = load_input();
    match load_input() {
        Ok(v) => println!("{}", v),
        _ => ,
    }
    for s in v {
        println!("{}", s);
    }
}
