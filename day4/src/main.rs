use std::{
    collections::{HashMap, HashSet},
    fs,
};
//horrendous performance
fn main() {
    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).unwrap();
    let card_list: Vec<Card> = content.lines().map(|line| Card::new(line)).collect();
    let result = card_list.iter().fold(0, |acc, card| acc + card.points);
    println!("Part 1 result: {}", result);
    let mut card_database: HashMap<i32, Vec<Card>> = Default::default();
    card_list
        .iter()
        .for_each(|card| card_database.entry(card.id).or_default().push(card.clone()));
    for key in 1..=card_list.len() as i32 {
        let mut original_cards = vec![];
        if let Some(cards) = card_database.get(&key) {
            original_cards = cards.clone()
        }
        original_cards
            .iter()
            .for_each(|card| card.process(&card_list, &mut card_database));
    }
    let part2_result: i32 = card_database
        .iter()
        .fold(0, |acc, (_, cards)| acc + cards.len() as i32);
    println!("{}", part2_result)
}
#[derive(Clone, Debug)]
struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    player_numbers: HashSet<i32>,
    matches: u32,
    points: i32,
}

impl Card {
    fn new(line: &str) -> Card {
        let data: Vec<Vec<&str>> = line
            .split([':', '|'])
            .map(|chunk| chunk.split_whitespace().collect())
            .collect();
        let winning_numbers: HashSet<i32> = data[1]
            .iter()
            .map(|&num| num.parse::<i32>().unwrap())
            .collect();
        let player_numbers: HashSet<i32> =
            data[2].iter().map(|&num| num.parse().unwrap()).collect();
        let matches = winning_numbers.intersection(&player_numbers).count() as u32;

        let points = if matches == 0 {
            0
        } else {
            2_i32.pow(matches.saturating_sub(1))
        };
        Card {
            id: data[0][1].parse().unwrap(),
            winning_numbers,
            player_numbers,
            matches,
            points,
        }
    }
    fn process(&self, card_list: &[Card], card_database: &mut HashMap<i32, Vec<Card>>) {
        for index in (self.id + 1)..=(self.id + self.matches as i32) {
            card_database
                .entry(index)
                .and_modify(|list| list.push(card_list[(index - 1) as usize].clone()));
        }
    }
}
