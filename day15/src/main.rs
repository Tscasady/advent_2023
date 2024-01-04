use core::str::Chars;
use std::collections::HashMap;
use std::collections::hash_map::Keys;

fn main() {
    let data = include_str!("../input.txt");
    println!("{}", part1(data));
    println!("{}", part2(data))
}

fn part1(data: &str) -> i32 {
    let sequences: Vec<&str> = data.trim_end().split(',').collect();
    sequences.iter().fold(0, |acc, sequence| acc + sequence_to_num(sequence.chars()))
}

fn part2(data: &str) -> i32 {
    let sequences: Vec<&str> = data.trim_end().split(',').collect();
    let instructions: Vec<Lens> = sequences.iter().map(|sequence| Lens::new(sequence)).collect();
    let mut boxes = Boxes::new();
    for instruction in instructions {
        match instruction.operation {
            Operation::Add => {
                boxes.entry(sequence_to_num(instruction.label.chars()), instruction);

            },
            Operation::Sub => {
                boxes.remove(sequence_to_num(instruction.label.chars()), instruction)
            },
        }
    }
    boxes.focusing_power()
}

fn sequence_to_num(sequence: Chars<'_>) -> i32 {
   sequence.fold(0, |acc, c| char_to_num(c, acc)) 
}

fn char_to_num(c: char, mut total: i32) -> i32 {
    total += c as u8 as i32;
    total *= 17;
    total %= 256;
    total
}

struct Boxes {
    data: HashMap<i32, Vec<Lens>>
}

impl Boxes {
    fn new() -> Self {
        Boxes {
            data: HashMap::default(),
        }
    }

    fn entry(&mut self, key: i32, value: Lens) {
        let lenses = self.data.entry(key).or_default();

        if let Some(lens) = lenses.iter().position(|item| item.label == value.label) {
            lenses[lens] = value;
        } else {
            lenses.push(value);
        }
    }

    fn remove(&mut self, key: i32, value: Lens) {
        if let Some(lenses) = self.data.get_mut(&key) {
            if let Some(index) = lenses.iter().position(|item| item.label == value.label) {
                lenses.remove(index); 
            }
        };
    }

    fn keys(&self) -> Keys<'_, i32, Vec<Lens>> {
        self.data.keys()
    }

    fn focusing_power(&self) -> i32 {
        let mut result = 0;
        for key in self.keys() {
            let box_num = *key + 1;
            for (lens_index, lens) in self.data.get(key).unwrap().iter().enumerate() {
                result += box_num * (lens_index as i32 + 1) * lens.focal_len.unwrap();
            }
        }
        result
    }
}

#[derive(Debug)]
struct Lens {
    label: String,
    operation: Operation,
    focal_len: Option<i32> 
}

impl Lens {
    fn new(data: &str) -> Self {
        let operation = if data.contains('=') {
            Operation::Add            
        } else { Operation::Sub };

        let data: Vec<&str> = data.split(['=', '-']).filter(|s| !s.is_empty()).collect();

        
        Lens {
            label: data[0].to_string(),
            operation,
            focal_len: data.get(1).map(|data| data.parse::<i32>().unwrap())
            
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash() {
        let data = "HASH";
        assert_eq!(part1(data), 52)
    }

    #[test]
    fn part1_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part1(data), 1320)
    }

    #[test]
    fn part2_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part2(data), 145)
    }
}
