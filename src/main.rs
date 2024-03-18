#![allow(dead_code, unused)]

mod kb;
mod keys;

use std::str::FromStr;

use kb::*;
use keys::*;

fn main() {
    // let keys: Vec<Key> = include_str!("../test_sample")
    // .lines()
    // .filter_map(|line| parse_line(line))
    // .collect();

    // println!("{:#?}", keys)

    let layout = include_str!("../us.layout");
    let kb = Keyboard::from_layout(layout).unwrap();
}

fn parse_line(line: impl ToString) -> Option<Key> {
    let line = line.to_string();
    let (_, key_str) = line.split_once(">")?;
    let key_str = key_str.trim();
    if key_str.is_empty() {
        return Some(Key::Space);
    }
    let key = Key::from_str(key_str).ok()?;
    Some(key)
}
