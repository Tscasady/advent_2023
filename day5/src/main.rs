use std::cmp::min;
use std::fs;
use std::ops::Range;

fn main() {
    let file_path = "./src/test.txt";
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
    let maps: Vec<Vec<Map>> = chunks[1..]
        .iter()
        .map(|chunk| chunk[1..].iter().map(|s| Map::new(s)).collect())
        .collect();
    let mut min_location = i64::MAX;
    for seed in seeds {
        for map in maps {
            min_location = min(min_location, get_location(seed))
        }
    }
    println!("{:?}", min_location)
}

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
            range: (numbers[1]..numbers[1] + numbers[2]),
            start: numbers[0],
        }
    }
}

fn get_location(seed: i64) -> i64 {
    todo!()
}
