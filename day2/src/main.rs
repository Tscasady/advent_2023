use std::fs;

fn main() {
    let file_path = "./input.txt";
    let content = fs::read_to_string(file_path).unwrap();
    let lines = content.lines();
    let result = lines
        .into_iter()
        .map(|line| Game::new_set(line))
        .fold(0, |acc: i32, game| acc + check_games(game));
    println!("{}", result)
}

fn check_games(mut game_set: Vec<Game>) -> i32 {
    let mut result = 0;
    while let Some(game) = game_set.pop() {
        if game.red > 12 || game.blue > 14 || game.green > 13 {
            return 0;
        } else {
            result = game.id
        }
    }
    result
}
#[derive(Debug)]
pub struct Game {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn new_set(line: &str) -> Vec<Self> {
        let mut set = vec![];
        let mut game_data: Vec<Vec<&str>> = line
            .split([':', ';'])
            .map(|chunk| {
                chunk
                    .split(',')
                    .flat_map(|s| s.split_whitespace().collect::<Vec<_>>())
                    .collect::<Vec<_>>()
            })
            .collect();
        let id = game_data[0][1].parse().ok().unwrap();
        while let Some(mut data) = game_data.pop() {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            while let Some(inner_data) = data.pop() {
                match inner_data {
                    "green" => green = data.pop().unwrap().parse().ok().unwrap(),
                    "red" => red = data.pop().unwrap().parse().ok().unwrap(),
                    "blue" => blue = data.pop().unwrap().parse().ok().unwrap(),
                    _ => {}
                }
            }
            if red != 0 || green != 0 || blue != 0 {
                set.push(Game {
                    id,
                    red,
                    blue,
                    green,
                })
            }
        }

        return set;
    }
}
