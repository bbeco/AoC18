#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
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
    corners: [[usize; 2]; 2],
}

impl Patch {
    fn new(p_id: u16, x: usize, y: usize, w: usize, h: usize) -> Self {
        Patch {
            id: p_id,
            corners: [[x, y], [x + w, y + h]],
        }
    }
    fn parse(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#(\d+)\s+@\s(\d+),(\d+):\s+(\d+)x(\d+)$").unwrap();
        }
        let caps = RE.captures(s).unwrap();

        let id = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
        let x = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let y = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let w = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let h = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

        Patch::new(id, x, y, w, h)
    }
}

fn intersect(p: &Patch, piece: &[[u8; 1000]; 1000]) -> bool {
    for i in p.corners[0][1]..p.corners[1][1] {
        for j in p.corners[0][0]..p.corners[1][0] {
            if piece[i][j] > 1 {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut piece: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
    let inputs = load_input().unwrap();
    let mut area = 0;
    let patches: Vec<Patch> = inputs.iter().map(|x| Patch::parse(x)).collect();
    for p in patches.iter() {
        for i in p.corners[0][1]..p.corners[1][1] {
            for j in p.corners[0][0]..p.corners[1][0] {
                if piece[i][j] == 1 {
                    area += 1;
                }
                if piece[i][j] < 2 {
                    piece[i][j] += 1;
                }
            }
        }
    }

    println!("area: {}", area);

    println!(
        "id of non intersecting patch: {}",
        patches.iter().find(|x| intersect(x, &piece)).unwrap().id
    );
}
