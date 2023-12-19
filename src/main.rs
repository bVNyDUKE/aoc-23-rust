use std::{fs, i64};

fn convert_value(val: i64, valmap: Vec<Vec<i64>>) -> i64 {
    for row in valmap {
        let (dest, src, rlen) = (row[0], row[1], row[2]);
        if src <= val && val < (src + rlen) {
            return val + (dest - src);
        }
    }

    return val;
}

fn main() {
    let content = fs::read_to_string("src/input.txt").unwrap();
    let (seed_line, maps) = content.split_once("\n").unwrap();

    let seeds: Vec<i64> = seed_line
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let lines: Vec<Vec<&str>> = maps
        .trim()
        .split("\n\n")
        .map(|el| el.trim().split("\n").collect())
        .collect();

    println!("{:?}", lines);
    println!("{:?}", seeds);
}
