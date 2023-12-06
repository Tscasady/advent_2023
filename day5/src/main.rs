use std::cmp::min;
use std::fs;
use std::ops::Range;

fn main() {
    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).unwrap();
    let chunks: Vec<Vec<&str>> = content
        .split("\n\n")
        .map(|chunk| chunk.lines().collect::<Vec<&str>>())
        .collect();
    let seeds: Vec<i64> = chunks[0][0]
        .split(':')
        .flat_map(|s| s.split_whitespace())
        .filter_map(|seed_data| seed_data.parse().ok())
        .collect();
    let maptypes: Vec<Vec<Map>> = chunks[1..]
        .iter()
        .map(|chunk| chunk[1..].iter().map(|s| Map::new(s)).collect())
        .collect();
    let mut min_location = i64::MAX;
    for seed in seeds {
        min_location = min(min_location, get_location(seed, &maptypes))
    }
    println!("{:?}", min_location)
}

#[derive(Debug)]
struct Map {
    range: Range<i64>,
    start: i64,
}

impl Map {
    fn new(data: &str) -> Self {
        let numbers: Vec<i64> = data
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        Map {
            range: (numbers[1]..(numbers[1] + numbers[2])),
            start: numbers[0],
        }
    }
}

fn get_location(seed: i64, maptypes: &Vec<Vec<Map>>) -> i64 {
    let mut temp = seed;
    for maps in maptypes {
        for map in maps {
            if map.range.contains(&temp) {
                temp = map.start + (temp - map.range.start);
                break;
            }
        }
    }
    temp
}
