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

fn foo(s: String) -> (u8, u8) {
    let mut leftover = s;
    let mut res: (u8, u8) = (0, 0);
    for c in leftover.clone().chars() {
        let mut count = 0;
        for _ in leftover.chars().filter(|x| *x == c) {
            count += 1;
            if count > 3 {
                break;
            }
        }

        if count == 2 {
            res.0 = 1;
        } else if count == 3 {
            res.1 = 1;
        }

        leftover = leftover.chars().filter(|x| *x != c).collect();
    }
    res
}

fn find_similar(strings: &Vec<String>) -> Option<(&String, &String)> {
    for i in 0..strings.len() - 1 {
        for j in i + 1..strings.len() {
            if count_differences(&strings[i], &strings[j]) < 2 {
                return Some((&strings[i], &strings[j]));
            }
        }
    }
    None
}

fn count_differences(a: &String, b: &String) -> u8 {
    let mut count: u8 = 0;
    for pair in a.chars().zip(b.chars()) {
        if pair.0 != pair.1 {
            count += 1;
            if count > 1 {
                break;
            }
        }
    }
    count
}

fn common_characters(a: &String, b: &String) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|p| p.0 == p.1)
        .map(|p| p.1)
        .collect()
}

fn main() {
    let strings = load_input().unwrap();
    let res = strings.iter().fold((0, 0), |acc, x| {
        let res = foo(x.to_string());
        return (acc.0 + res.0 as u32, acc.1 + res.1 as u32);
    });

    println!("res: {}\nstrings number: {}", res.0 * res.1, strings.len());

    let ids = find_similar(&strings);
    match ids {
        Some(words) => println!("common chars: {}", common_characters(words.0, words.1)),
        _ => (),
    }
}
