use regex::Regex;
use std::collections::HashMap;
use std::fs;
fn main() {
    let file_path = "./input.txt";
    let binding = fs::read_to_string(file_path).unwrap();
    let content = binding.lines();

    let mut hash = HashMap::new();
    hash.insert("1", 1);
    hash.insert("2", 2);
    hash.insert("3", 3);
    hash.insert("4", 4);
    hash.insert("5", 5);
    hash.insert("6", 6);
    hash.insert("7", 7);
    hash.insert("8", 8);
    hash.insert("9", 9);

    hash.insert("one", 1);
    hash.insert("two", 2);
    hash.insert("three", 3);
    hash.insert("four", 4);
    hash.insert("five", 5);
    hash.insert("six", 6);
    hash.insert("seven", 7);
    hash.insert("eight", 8);
    hash.insert("nine", 9);

    let mut numbers = vec![];
    for line in content {
        let mut digits = (0, 0);
        'outer: for length in 1..=line.len() {
            let substring = &line[0..length];
            for (key, val) in &hash {
                if substring.contains(key) {
                    digits.0 = val * 10;
                    break 'outer;
                }
            }
        }
        'outer: for length in 1..=line.len() {
            let substring = &line[line.len() - length..];
            println!("{}", substring);
            for (key, val) in &hash {
                if substring.contains(key) {
                    digits.1 = *val;
                    break 'outer;
                }
            }
        }
        numbers.push(digits);
    }

    let result = numbers.iter().fold(0, |acc, digits| {
        let first = digits.0;
        let last = digits.1;
        acc + first + last
    });
    println!("{}", result)
}

pub fn part_1() {
    let file_path = "day1/input.txt";
    let binding = fs::read_to_string(file_path).unwrap();
    let content = binding.lines();
    let numbers: Vec<Vec<char>> = content
        .map(|line| {
            line.chars()
                .filter(|char| char.is_numeric())
                .collect::<Vec<char>>()
        })
        .collect();
    let mut result = 0;
    for line in numbers {
        let num: u32 = match line.len() {
            0 => 0,
            _ => {
                let first = line.first().unwrap();
                let last = line.last().unwrap();
                (first.to_string() + &last.to_string())
                    .parse::<u32>()
                    .ok()
                    .unwrap()
            }
        };

        result += num
    }
    println!("{}", result)
}
