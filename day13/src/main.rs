use std::cmp::min;

fn main() {
    let data = include_str!("../input.txt");

    println!("{}", part1(data));
}

fn part1(data: &'static str) -> usize {
    let patterns: Vec<Pattern> = data.split("\n\n").map(Pattern::new).collect();
    println!("{:?}", patterns);
    patterns.iter().fold(0, |acc, pattern| acc + pattern.test())
}

#[derive(Debug)]
#[allow(dead_code)]
struct Pattern {
    data: Vec<&'static str>,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
impl Pattern {
    fn new(data: &'static str) -> Self {
        let data: Vec<&str> = data.lines().collect();
        let width = data[0].len();
        let height = data.len();

        Pattern {
            data,
            width,
            height,
        }
    }

    fn test(&self) -> usize {
        if let Some(index) = self.test_vertical() {
            index
        } else {
            100 * self.test_horizontal()
        }
    }

    fn test_vertical(&self) -> Option<usize> {
        let mut possible_indices: Vec<usize> = (1..self.width).collect();
        for line in self.data.iter() {
            possible_indices.retain(|index| Self::vertical_check(index, line))
        }
        possible_indices.last().copied()
    }

    fn test_horizontal(&self) -> usize {
        let mut possible_indices: Vec<usize> = (1..self.height).collect();
        possible_indices.retain(|index| self.horizontal_check(index));
        possible_indices.last().copied().unwrap()
    }

    fn horizontal_check(&self, index: &usize) -> bool {
        let fold_size = min(*index, self.height - *index);
        let mut check = true;
        for offset in 1..=fold_size {
            if self.data[index - offset] != self.data[index - 1 + offset] {
                check = false
            }
        }
        check
    }

    fn vertical_check(index: &usize, data: &str) -> bool {
        let fold_size = min(*index, data.len() - *index);
        let left: Vec<char> = data[index - fold_size..*index].chars().collect();
        let right: Vec<char> = data[*index..*index + fold_size].chars().rev().collect();
        left == right
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part1(data), 405)
    }
}
