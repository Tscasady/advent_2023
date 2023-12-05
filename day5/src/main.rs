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
    println!("{:?}", seeds)
}

struct Map {
    range: Range<i64>,
    start: i64,
}

impl Map {
    fn new(data: &str) -> Self {
        todo!()
    }
}
