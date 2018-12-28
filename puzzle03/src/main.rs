#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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

struct Patch {
    id: u16,
    corners: [[u32; 2]; 2],
}

impl Patch {
    fn new(p_id: u16, x: u32, y: u32, w: u32, h: u32) -> Self {
        Patch {
            id: p_id,
            corners: [[x, y], [x + w, y + h]],
        }
    }
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+)\s+@\s(\d+),(\d+):\s+(\d+)x(\d+)$").unwrap();
        }
        let re = Regex::new(r"^#(\d+)\s+@\s(\d+),(\d+):\s+(\d+)x(\d+)$").unwrap();
        let caps = re.captures(s).unwrap();

        let x = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let y = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let w = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
        let h = caps.get(5).unwrap().as_str().parse::<u32>().unwrap();

        Patch::new(0, x, y, w, h)
    }

    fn intersect(&self, other: &Patch) -> Patch {
        let x1 = cmp::max(self.corners[0][0], other.corners[0][0]);
        let y1 = cmp::max(self.corners[0][1], other.corners[0][1]);
        let x2 = cmp::min(self.corners[1][0], other.corners[1][0]);
        let y2 = cmp::min(self.corners[1][1], other.corners[1][1]);
        Patch {
            id: 0,
            corners: [[x1, y1], [x2, y2]],
        }
    }

    fn area(&self) -> u32 {
        (self.corners[1][0] - self.corners[0][0]) * (self.corners[1][1] - self.corners[0][1])
    }
}

fn main() {
    let p = Patch::new(0, 0, 0, 1000, 1000);
    let inputs = load_input().unwrap();
    // let inputs = ["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
    let res = inputs
        .iter()
        .fold(0, |acc, x| cmp::max(acc, Patch::parse(x).corners[1][0]));

    println!("area: {}", res);
}
