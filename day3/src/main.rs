use std::{fs, i32};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).unwrap();
    let matrix: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let result = get_parts(&matrix)
        .iter()
        .fold(0, |acc, part| acc + part.value);

    println!("{}", result)
}

fn part_two() {
    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).unwrap();
    let matrix: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let parts = get_parts(&matrix);

    let mut result = 0;
    for (y, line) in matrix.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            match ch {
                '*' => {
                    let gear_parts = get_gear_parts(x, y, &parts);
                    if gear_parts.len() == 2 {
                        result += gear_parts.iter().fold(1, |acc, gear| acc * gear.value)
                    }
                }
                _ => continue,
            }
        }
    }
    println!("{}", result)
}

fn get_parts(matrix: &Vec<Vec<char>>) -> Vec<Part> {
    let mut prev: Option<char> = None;
    let mut current_number: Vec<char> = vec![];
    let mut valid = false;
    let mut result: Vec<Part> = vec![];
    for (y, line) in matrix.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            match ch {
                ch if ch.is_digit(10) => {
                    current_number.push(*ch);
                    if !valid {
                        valid = check(x, y, &matrix);
                    }
                    prev = Some(*ch)
                }
                _ => {
                    if prev.is_some() {
                        if valid {
                            result.push(Part {
                                left_index: x - current_number.len(),
                                right_index: x - 1,
                                y,
                                value: current_number
                                    .clone()
                                    .into_iter()
                                    .collect::<String>()
                                    .parse::<i32>()
                                    .ok()
                                    .unwrap(),
                            })
                        }
                        current_number.drain(..);
                        valid = false;
                        prev = None
                    }
                }
            }
        }
        if valid {
            result.push(Part {
                left_index: line.len() - current_number.len(),
                right_index: line.len() - 1,
                y,
                value: current_number
                    .clone()
                    .into_iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .ok()
                    .unwrap(),
            })
        }
        valid = false;
        prev = None;
        current_number.drain(..);
    }
    result
}

fn check(x: usize, y: usize, matrix: &Vec<Vec<char>>) -> bool {
    for x_mod in -1..=1 {
        for y_mod in -1..=1 {
            let checked_y = y.checked_add_signed(y_mod);
            let checked_x = x.checked_add_signed(x_mod);
            if let (Some(target_x), Some(target_y)) = (checked_x, checked_y) {
                if target_y < matrix.len() {
                    let row = &matrix[target_y];
                    if target_x < row.len() {
                        match row[target_x] {
                            '@' | '#' | '$' | '%' | '&' | '*' | '+' | '-' | '=' | '/' => {
                                return true
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
    false
}

fn get_gear_parts(x: usize, y: usize, parts: &[Part]) -> Vec<&Part> {
    parts
        .iter()
        .filter(|part| {
            //this is advanced gross
            //i already dont know what it means
            (part.y.saturating_sub(1)..=part.y.saturating_add(1)).contains(&y)
                && (part.left_index.saturating_sub(1)..=part.right_index.saturating_add(1))
                    .contains(&x)
        })
        .collect()
}

// fn check_gear(x: usize, y: usize, matrix: &Vec<Vec<char>>) -> i32 {
//     let gears = vec![];
//     for x_mod in -1..=1 {
//         for y_mod in -1..=1 {
//             let checked_y = y.checked_add_signed(y_mod);
//             let checked_x = x.checked_add_signed(x_mod);
//             if let (Some(target_x), Some(target_y)) = (checked_x, checked_y) {
//                 if target_y < matrix.len() {
//                     let row = &matrix[target_y];
//                     if target_x < row.len() {
//                         match row[target_x] {
//                             '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
//                                 gear = get_number(target_x, target_y, matrix)
//                             }
//
//                             _ => continue,
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     if gears.len() > 2 {
//         return 0;
//     } else {
//         gears.iter().sum()
//     }
// }
#[derive(Debug)]
struct Part {
    left_index: usize,
    right_index: usize,
    y: usize,
    value: i32,
}

impl Part {
    fn new(left_index: usize, right_index: usize, y: usize, digits: String) -> Self {
        Part {
            left_index,
            right_index,
            y,
            value: digits.parse::<i32>().ok().unwrap(),
        }
    }
}

// fn get_number(x: usize, y: usize, matrix: &Vec<Vec<char>>) -> Part {
//     let left_x = x.checked_add_signed(-1);
//     let right_x = x.checked_add_signed(-1);
//     let part = None;
//     let mut right_index = y;
//     let mut left_index = x;
//     let mut left_digits = "".to_string();
//     let mut right_digits = "".to_string();
//     if let Some(x) = left_x {
//         if matrix[y][x].is_digit(10) {
//             (left_index, left_digits) = get_left(x, y, &matrix)
//         }
//     }
//     if let Some(x) = right_x {
//         if matrix[y][x].is_digit(10) {
//             (right_index, right_digits) = get_right(x, y, &matrix)
//         }
//     }
//     let value = format!("{}{}{}", left_digits, matrix[y][x], right_digits);
//     Part::new(left_index, right_index, y, value)
// }
//
// fn get_left(x: usize, y: usize, matrix: &Vec<Vec<char>>) -> (usize, String) {
//     let left_x = x.checked_add_signed(-1);
//     let right_x = x.checked_add_signed(-1);
//     if matrix[y][x].is_digit(10) {
//         (left_index, left_digits) = get_left(x, y, &matrix)
//     }
// }
