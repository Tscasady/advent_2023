fn main() {
    let platform = parse_input(include_str!("../input.txt"));
    println!("{}", part1(platform))
}

fn parse_input(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect()).collect()
}

fn part1(platform: Vec<Vec<char>>) -> i32 {
    let mut queues: Vec<Vec<char>> = vec![Vec::new(); platform.len()];
    for (row_index, row) in platform.iter().enumerate() {
        for (index, item) in row.iter().enumerate() {
            match item {
                'O' => queues[index].push(*item),
                '.' => continue,
                '#' => {
                    while queues[index].len() < row_index {
                        queues[index].push('.')
                    }
                    queues[index].push(*item)
                }
                _ => unreachable!(),
            }
        }
    }

    queues.iter().fold(0, |acc, queue| {
        acc + queue
            .iter()
            .enumerate()
            .fold(0, |sum, (index, char)| match char {
                'O' => sum + (platform.len() - index) as i32,
                _ => sum,
            })
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let data = parse_input(include_str!("./test.txt"));
        assert_eq!(part1(data), 136)
    }
}
