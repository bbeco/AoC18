#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
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

struct Time {
    year: u16,
    month: u8, // [0, 11]
    day: u8,   // [1, 31]
    hours: u8,
    minutes: u8, // [00 - 59]
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}-{}-{} {}:{}]",
            self.year, self.month, self.day, self.hours, self.minutes
        )
    }
}

enum Action {
    Begin(u16),
    Asleep,
    Awake,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Begin(i) => write!(f, "Begin({})", i),
            Action::Asleep => write!(f, "Asleep"),
            Action::Awake => write!(f, "Awake"),
        }
    }
}

type Entry = (Time, Action);
type Range = [u8; 2];

fn parse_entry(s: &str) -> Entry {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^\[(\d{4})-(\d{2})-(\d{2})\s+(\d{2}):(\d{2})\]\s([^\s\n]{5})\s(.*)$")
                .unwrap();
    }
    let caps = RE.captures(s).unwrap();

    let y = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
    let m = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();
    let d = caps.get(3).unwrap().as_str().parse::<u8>().unwrap();
    let h = caps.get(4).unwrap().as_str().parse::<u8>().unwrap();
    let min = caps.get(5).unwrap().as_str().parse::<u8>().unwrap();

    let action: Action;
    let command = caps.get(6).unwrap().as_str();
    if command == "Guard" {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"^#(\d+).*$").unwrap();
        }
        let action_caps = ID_RE.captures(caps.get(7).unwrap().as_str()).unwrap();
        action = Action::Begin(action_caps.get(1).unwrap().as_str().parse::<u16>().unwrap());
    } else if command == "falls" {
        action = Action::Asleep;
    } else {
        action = Action::Awake;
    }

    (
        Time {
            year: y,
            month: m,
            day: d,
            hours: h,
            minutes: min,
        },
        action,
    )
}

fn main() {
    let mut inputs = load_input().unwrap();
    inputs.sort();
    let entries = inputs
        .iter()
        .map(|x| parse_entry(x))
        .collect::<Vec<Entry>>();

    let mut map: HashMap<u16, [usize; 60]> = HashMap::new();
    let mut curr_id = 0;
    let mut curr_start = 0;
    for e in entries.iter() {
        match e.1 {
            Action::Begin(id) => curr_id = id,
            Action::Asleep => curr_start = e.0.minutes,
            Action::Awake => {
                let mins = map.get_mut(&curr_id);
                match mins {
                    Some(v) => {
                        for i in curr_start..e.0.minutes {
                            v[i as usize] += 1;
                        }
                    }
                    None => {
                        let tmp: [usize; 60] = [0; 60];
                        map.insert(curr_id, tmp);
                    }
                }
            }
        }
    }

    let id_max = map
        .iter()
        .map(|(k, v)| (k, v.iter().sum::<usize>()))
        .max_by(|a, b| {
            if a.1 < b.1 {
                return Ordering::Less;
            } else if a.1 == b.1 {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        })
        .unwrap()
        .0;

    let minute_max = map
        .get(id_max)
        .unwrap()
        .iter()
        .enumerate()
        .max_by_key(|&(_, val)| val)
        .unwrap()
        .0;

    let product = *id_max as u32 * minute_max as u32;
    println!("{} * {} = {}", id_max, minute_max, product);
}
