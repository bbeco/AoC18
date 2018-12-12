use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn load_input() -> std::io::Result<Vec<String>> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let mut v: Vec<String> = vec![];
    for res in buf_reader.lines() {
        let s = res.unwrap();
        let trimmed = String::from(s.trim());
        v.push(trimmed);
    }
    Ok(v)
}

fn main() {
    let v = load_input().unwrap();
    for s in v {
        println!("{}", s);
    }
}
