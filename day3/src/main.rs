use std::fs;

fn main() {
    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).unwrap();
    let matrix: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let mut prev: Option<char> = None;
    let mut current_number: Vec<char> = vec![];
    let mut valid = false;
    let mut result = 0;
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
                            println!(
                                "{:?}",
                                current_number
                                    .clone()
                                    .into_iter()
                                    .collect::<String>()
                                    .parse::<i32>()
                                    .ok()
                                    .unwrap()
                            );
                            result += current_number
                                .clone()
                                .into_iter()
                                .collect::<String>()
                                .parse::<i32>()
                                .ok()
                                .unwrap();
                        }
                        current_number.drain(..);
                        valid = false;
                        prev = None
                    }
                }
            }
        }
        if valid {
            result += current_number
                .clone()
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .ok()
                .unwrap();
        }
        valid = false;
        prev = None;
        current_number.drain(..);
    }
    println!("{}", result)
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
