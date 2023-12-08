use std::{cmp::Ordering, collections::HashMap, char};

fn main() {
    let mut hands: Vec<Hand> = include_str!("../input.txt")
        .lines()
        .map(Hand::new)
        .collect();
    hands.sort();
    let mut result = 0;
    for (i, hand) in hands.iter().enumerate() {
        result += (i + 1) * hand.bid as usize 
    }
    println!("{}", result)
}


#[derive(PartialEq, Eq)]
struct Hand {
    bid: u32,
    kind: HandType,
    hand: Vec<u32>
}


#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five
}

impl Hand {
    fn new(line: &str) -> Self {
        let data: Vec<&str> = line.split_whitespace().collect();

        Hand {
            bid: data[1].parse().unwrap(),
            kind: HandType::new(data[0]),
            hand: Hand::parse(data[0]) 
        }
    }
    fn compare(&self, other: &Self) -> Ordering {
        for i in 0..=self.hand.len() {
            if self.hand[i] != other.hand[i] {
                return self.hand[i].cmp(&other.hand[i])
            } 
        }
        Ordering::Equal
    } 
    fn parse(data: &str) -> Vec<u32> {
        data.chars().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                c => c.to_digit(10).unwrap()
            }
        }).collect()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.kind.cmp(&other.kind);

        match type_order {
            Ordering::Equal => {
                self.compare(other)
            }
            _ => type_order,
        }
    }
}

impl HandType {
    fn new(data: &str) -> Self {
        let mut hash: HashMap<char, u32> = Default::default(); 

        data.chars().for_each(|c| *hash.entry(c).or_insert(0) += 1);
        match hash.values().max().unwrap() {
            5 => HandType::Five,
            4 => HandType::Four,
            3 => if hash.values().any(|value| *value == 2) {
                HandType::FullHouse
            } else {
                HandType::Three
            },
            2 => if hash.values().filter(|&&value| value == 2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::Pair
            },
            1 => HandType::HighCard,
            _ => unreachable!()
        }
    }
}
