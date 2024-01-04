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
    let s = Loop::new(&matrix);
    s.enclosed_area()
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
            _ => unreachable!(),
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
            Direction::Invalid => Direction::Invalid,
        }
    }
}

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

impl From<i32> for Rotation {
    fn from(value: i32) -> Self {
        if value < 0 {
            Self::Left
        } else {
            Self::Right
        }
    }
}

impl Rotation {
    fn derive(pipe: char, direction: Direction) -> i32 {
        match pipe {
            '|' => 0,
            '-' => 0,
            'L' => match direction {
                Direction::S => -1,
                Direction::W => 1,
                _ => unreachable!(),
            },
            'J' => match direction {
                Direction::E => -1,
                Direction::S => 1,
                _ => unreachable!(),
            },

            '7' => match direction {
                Direction::E => 1,
                Direction::N => -1,
                _ => unreachable!()
            }
            'F' => match direction {
                Direction::N => 1,
                Direction::W => -1,
                _ => unreachable!()
            }
            _ => 0,
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
            }
            Direction::S => {
                let x = self.x;
                let y = self.y.checked_add_signed(1);
                y.map(|y| Coord { x, y })
            }
            Direction::E => {
                let y = self.y;
                let x = self.x.checked_add_signed(1);
                x.map(|x| Coord { x, y })
            }
            Direction::W => {
                let y = self.y;
                let x = self.x.checked_add_signed(-1);
                x.map(|x| Coord { x, y })
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Loop {
    head: Coord,
    len: i32,
    rotation: Rotation,
    cells: Vec<Cell>
}

impl Loop {
    fn new(matrix: &Vec<Vec<char>>) -> Self {
        let coords = Self::find_start(matrix);
        let mut cells = vec![];
        let (len, rotation) = Self::find_loop(coords, matrix, &mut cells);
        let rotation = Rotation::from(rotation);
        Loop {
            head: coords.clone(),
            len,
            rotation,
            cells
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

    fn find_loop(coords: Coord, matrix: &Vec<Vec<char>>, loop_cells: &mut Vec<Cell>) -> (i32, i32) {
        let mut start = Cell::start(coords);
        while start.curr_len == 0 || start.going_to != Direction::Any {
            start = start.next(matrix, loop_cells);
        }
        (start.curr_len, start.rotation)
    }

    fn enclosed_area(self) -> i32 {
        let polygon: HashMap<usize, Vec<(usize, usize)>> = HashMap::default();
        //walk the loop
        //for each step, get row, 
        match self.rotation {
            Rotation::Left => {},
            Rotation::Right => {}
        }

    }
}

#[derive(Debug)]
struct Cell {
    curr_len: i32,
    coord: Coord,
    going_to: Direction,
    rotation: i32,
}

impl Cell {
    pub const DIRECTIONS: [Direction; 4] = [Direction::N, Direction::S, Direction::E, Direction::W];

    fn new(prev_cell: &Cell, matrix: &Vec<Vec<char>>, loop_cells: &mut Vec<Cell>) -> Option<Self> {
        let coord = prev_cell.coord.move_coord(&prev_cell.going_to);
        coord.map(|coord| {
            let pipe: Pipe = matrix[coord.y][coord.x].into();
            let going_to = pipe.get_direction(prev_cell.going_to.opposite());
            let rotation = Rotation::derive(matrix[coord.y][coord.x], prev_cell.going_to);
            let cell = Cell {
                curr_len: prev_cell.curr_len + 1,
                coord,
                going_to,
                rotation: prev_cell.rotation + rotation,
            };
            loop_cells.push(cell);
            cell
        })
    }

    fn start(coord: Coord) -> Self {
        Cell {
            curr_len: 0,
            coord,
            going_to: Direction::Any,
            rotation: 0,
        }
    }

    fn next(&mut self, matrix: &Vec<Vec<char>>, loop_cells: &mut Vec<Cell>) -> Cell {
        match self.going_to {
            Direction::Any => {
                for dir in Self::DIRECTIONS {
                    self.going_to = dir;
                    if let Some(cell) = Cell::new(self, matrix, loop_cells) {
                        if cell.going_to == Direction::Invalid {
                            continue;
                        } else {
                            return cell;
                        }
                    };
                }
                unreachable!()
            }
            _ => Cell::new(self, matrix, loop_cells).unwrap(),
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
