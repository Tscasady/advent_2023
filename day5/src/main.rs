use std::cmp::min;
use std::collections::HashSet;
use std::fs;
use std::hash::Hasher;
use std::ops::Range;

fn main() {
    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).unwrap();
    let chunks: Vec<Vec<&str>> = content
        .split("\n\n")
        .map(|chunk| chunk.lines().collect::<Vec<&str>>())
        .collect();
    let seeds = get_seeds_part1(&chunks);
    let seeds_part_two = get_unique_seeds(&chunks);
    let maptypes: Vec<Vec<Map>> = chunks[1..]
        .iter()
        .map(|chunk| chunk[1..].iter().map(|s| Map::new(s)).collect())
        .collect();
    let mut min_location = i64::MAX;
    for seed in seeds {
        min_location = min(min_location, get_location(seed, &maptypes))
    }
    let mut min_location_part_two = i64::MAX;
    for seed in seeds_part_two {
        min_location_part_two = min(min_location_part_two, get_location(seed, &maptypes))
    }
    println!("{:?}", min_location);
    println!("{:?}", min_location_part_two)
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

fn get_seeds_part1(chunks: &Vec<Vec<&str>>) -> Vec<i64> {
    chunks[0][0]
        .split(':')
        .flat_map(|s| s.split_whitespace())
        .filter_map(|seed_data| seed_data.parse().ok())
        .collect()
}

fn get_seeds_part2(chunks: &Vec<Vec<&str>>) -> Vec<i64> {
    let range_values: Vec<i64> = chunks[0][0]
        .split(':')
        .flat_map(|s| s.split_whitespace())
        .filter_map(|seed_data| seed_data.parse().ok())
        .collect();
    let mut result = vec![];
    for i in (0..range_values.len() / 2) {
        let index = i * 2;
        let start = range_values[index];
        let range_length = range_values[index + 1];
        result.extend(start..start + range_length)
    }
    result
}

fn get_unique_seeds(chunks: &[Vec<&str>]) -> HashSet<i64> {
    let range_values: Vec<i64> = chunks[0][0]
        .split(':')
        .flat_map(|s| s.split_whitespace())
        .filter_map(|seed_data| seed_data.parse().ok())
        .collect();
    let mut result = vec![];
    for i in (0..range_values.len() / 2) {
        let index = i * 2;
        let start = range_values[index];
        let range_length = range_values[index + 1];
        result.extend(start..start + range_length)
    }

    result.into_iter().collect()
}
