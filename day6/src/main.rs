use std::cmp::Ordering;

fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.split(':')
                .flat_map(|s| s.split_whitespace())
                .filter_map(|digits| digits.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let races: Vec<Race> = lines[0]
        .iter()
        .zip(lines[1].iter())
        .map(|(a, b)| Race::new(*a, *b))
        .collect();

    let result = races.iter().fold(1, |acc, race| acc * race.number_of_wins());
    println!("{}", result)
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn new(time: i64, distance: i64) -> Self {
        Race { time, distance }
    }

    fn number_of_wins(&self) -> i64 {
        for num in 0..=self.time {
            let result = num * (self.time - num);
            match result.cmp(&self.distance) {
                Ordering::Equal => {
                    return (self.time + 1) - ((num * 2) + 2)},
                Ordering::Greater => {
                    return (self.time + 1) - (((num - 1) * 2) + 2)},
                _ => continue
            }
        }
        unreachable!()
    }

    // fn numbers_of_wins(&self) -> i32 {
    //     self.distance - (self.factor() + 1) * 2
    // }

    // fn factor(&self) -> i32 {
    //     let square = (self.distance as f64).sqrt() as i32;
    //     println!("{}", square);
    //     for num in 1..=square {
    //         if self.distance % num == 0 {
    //             let factor = self.distance / num;
    //             if num + factor == self.time {
    //                 return num;
    //             }
    //         }
    //     }
    //     unreachable!()
    // }
}

