use std::{collections::HashMap, str::FromStr};

fn main() {
    let content: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    println!("{}", part1(content.clone()));
    println!("{}", part2(content));
}

fn part1(matrix: Vec<Vec<char>>) -> i32 {
    let s = Loop::new(&matrix);
    s.len / 2
}

fn part2(matrix: Vec<Vec<char>>) -> i32 {
    todo!()
}

struct Pipe(Direction, Direction);

impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Pipe(Direction::N, Direction::S),
            '-' => Pipe(Direction::E, Direction::W),
            'L' => Pipe(Direction::N, Direction::E),
            'J' => Pipe(Direction::N, Direction::W),
            '7' => Pipe(Direction::W, Direction::S),
            'F' => Pipe(Direction::S, Direction::E),
            '.' => Pipe(Direction::Invalid, Direction::Invalid),
            'S' => Pipe(Direction::Any, Direction::Any),
            _ => unreachable!()
        }
    }
}

impl Pipe {
    fn get_direction(self, direction: Direction) -> Direction {
        match (self.0, self.1) {
            (d1, d2) if d1 == direction => d2,
            (d1, d2) if d2 == direction => d1,
            (Direction::Any, Direction::Any) => Direction::Any,
            (Direction::Invalid, Direction::Invalid) => Direction::Invalid,
            _ => Direction::Invalid,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    N,
    S,
    E,
    W,
    Any,
    Invalid,
}

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
            Direction::E => Direction::W,
            Direction::Any => Direction::Any,
            Direction::Invalid => Direction::Invalid
        }
    }
}

#[derive(Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(coords: (usize, usize)) -> Self {
        Coord {
            x: coords.0,
            y: coords.1,
        }
    }

    fn move_coord(&self, dir: &Direction) -> Option<Self> {
        match dir {
            Direction::N => {
                let x = self.x;
                let y = self.y.checked_add_signed(-1);
                y.map(|y| Coord { x, y })
            },
            Direction::S => {
                let x = self.x;
                let y = self.y.checked_add_signed(1);
                y.map(|y| Coord { x, y })
            },
            Direction::E => {
                let y = self.y;
                let x = self.x.checked_add_signed(1);
                x.map(|x| Coord { x, y })
            },
            Direction::W => {
                let y = self.y;
                let x = self.x.checked_add_signed(-1);
                x.map(|x| Coord { x, y })
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Loop {
    head: Coord,
    len: i32,
}

impl Loop {
    fn new(matrix: &Vec<Vec<char>>) -> Self {
        let coords = Self::find_start(matrix);
        Loop {
            head: coords.clone(),
            len: Self::find_len(coords, matrix),
        }
    }

    fn find_start(matrix: &Vec<Vec<char>>) -> Coord {
        for (i, row) in matrix.iter().enumerate() {
            if let Some(j) = row.iter().position(|&elem| elem == 'S') {
                return Coord::new((j, i));
            }
        }
        unreachable!()
    }

    fn find_len(coords: Coord, matrix: &Vec<Vec<char>>) -> i32 {
        let mut start = Cell::start(coords);
        while start.curr_len == 0 || start.going_to != Direction::Any {
            start = start.next(matrix);
        }
        start.curr_len
    }
}

#[derive(Debug)]
struct Cell {
    curr_len: i32,
    coord: Coord,
    going_to: Direction,
}

impl Cell {
    pub const DIRECTIONS: [Direction; 4] = [Direction::N, Direction::S, Direction::E, Direction::W];

    //BIG MESS
    fn new(prev_cell: &Cell, matrix: &Vec<Vec<char>>) -> Option<Self> {
        let coord = prev_cell.coord.move_coord(&prev_cell.going_to);
        coord.map(|coord| {
            let pipe: Pipe = matrix[coord.y][coord.x].into();
            let going_to = pipe.get_direction(prev_cell.going_to.opposite());

            Cell {
                curr_len: prev_cell.curr_len + 1,
                coord,
                going_to
            }
        })
    }

    fn start(coord: Coord) -> Self {
        Cell {
            curr_len: 0,
            coord,
            going_to: Direction::Any,
        }
    }

    fn next(&mut self, matrix: &Vec<Vec<char>>) -> Cell {
        match self.going_to {
            Direction::Any => {
                for dir in Self::DIRECTIONS {
                    self.going_to = dir;
                    if let Some(cell) = Cell::new(self, matrix) {
                        if cell.going_to == Direction::Invalid {
                            continue;
                        } else { return cell }
                    };
                }
                unreachable!()
            }
            _ => Cell::new(self, matrix).unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input: Vec<Vec<char>> = include_str!("./test.txt")
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        assert_eq!(part1(input), 8)
    }

    #[test]
    fn part2_test() {}
}
