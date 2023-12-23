use std::{fs, i64};
use tokio::task::JoinSet;

fn convert_value(val: i64, valmap: &Vec<Vec<i64>>) -> i64 {
    for row in valmap {
        let (dest, src, rlen) = (row[0], row[1], row[2]);
        if src <= val && val < (src + rlen) {
            return val + (dest - src);
        }
    }

    return val;
}

fn parse_map(lines: &Vec<Vec<&str>>, idx: usize) -> Vec<Vec<i64>> {
    let x: Vec<Vec<i64>> = lines
        .get(idx)
        .unwrap()
        .iter()
        .filter(|x| !x.contains("map"))
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    return x;
}

#[tokio::main]
async fn main() {
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

    let soilmap = parse_map(&lines, 0);
    let fertmap = parse_map(&lines, 1);
    let watermap = parse_map(&lines, 2);
    let lightmap = parse_map(&lines, 3);
    let tempmap = parse_map(&lines, 4);
    let humidmap = parse_map(&lines, 5);
    let locmap = parse_map(&lines, 6);

    let seed_location = move |seed: i64| -> i64 {
        let soil = convert_value(seed, &soilmap);
        let fert = convert_value(soil, &fertmap);
        let water = convert_value(fert, &watermap);
        let light = convert_value(water, &lightmap);
        let temp = convert_value(light, &tempmap);
        let hum = convert_value(temp, &humidmap);
        let loc = convert_value(hum, &locmap);
        return loc;
    };

    let mut set = JoinSet::new();

    let mut idx = 0;
    while idx < seeds.len() {
        let (start, rng) = (seeds[idx], seeds[idx+1]);
        let end = rng + start;
        let mid = (start +end) / 2;
        let seeds_f = seed_location.clone();
        let seed_s = seed_location.clone();
        set.spawn(async move {
            let mut res = -1;
            for s in start..mid{
                let val = seeds_f(s);
                if res == -1 || val < res {
                    res = val
                }
            }
            return res;
        });
        set.spawn(async move {
            let mut res = -1;
            for s in mid+1..end{
                let val = seed_s(s);
                if res == -1 || val < res {
                    res = val
                }
            }
            return res;
        });
        idx += 2;
    }

    let mut f: i64 = -1;
    while let Some(val) = set.join_next().await {
        let c = val.unwrap();
        if f == -1 || c < f {
            f = c
        }
    }

    println!("{}", f)

}
