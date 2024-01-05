use std::collections::HashMap;

fn main() {
    let data = include_str!("../input.txt");
    println!("{}", part1(data));
    println!("{}", part2(data));
}

fn part1(data: &str) -> usize {
    let matrix: Vec<Vec<Cell>> = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, symbol)| Cell::new(symbol, col, row))
                .collect()
        })
        .collect();
    let mut energized_cells: HashMap<Coord, Direction> = HashMap::default();
    energize(&matrix[0][0], Direction::E, &matrix, &mut energized_cells);
    energized_cells.len()
}

fn part2(data: &str) -> usize {
    let matrix: Vec<Vec<Cell>> = data
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, symbol)| Cell::new(symbol, col, row))
                .collect()
        })
        .collect();
    let mut results: Vec<usize> = vec![];
    for (row_index, _row) in matrix.iter().enumerate() {
        for (col_index, _col) in matrix.iter().enumerate() {
            if row_index == 0 || row_index == matrix.len() - 1 || col_index == 0 || col_index == matrix[0].len() -1 {
                let dirs = get_start_dirs(col_index, row_index, &matrix);
                println!("{:?}", dirs);
                for dir in dirs {
                    let mut energized_cells: HashMap<Coord, Direction> = HashMap::default();
                    println!("{:?}", energized_cells);
                    energize(&matrix[row_index][col_index], dir, &matrix, &mut energized_cells);
                    println!("{:?}", results);
                    results.push(energized_cells.len())
                }
            }  
        }
    };
    *results.iter().max().unwrap()
}

fn get_start_dirs(x: usize, y: usize, matrix: &Vec<Vec<Cell>>) -> Vec<Direction> {
    let width = matrix[0].len() - 1;
    let height = matrix.len() - 1;
    println!("{x}, {y}");
    
    match (x, y) {
        (0, 0) => vec![Direction::E, Direction::S],
        (0, y) if y == height => vec![Direction::N, Direction::E],
        (x, 0) if x == width => vec![Direction::W, Direction::S],
        (x, y) if x == width && y == height => vec![Direction::W, Direction::N],
        (0, _) => vec![Direction::E],
        (_, 0) => vec![Direction::S],
        (_, y) if y == height  => vec![Direction::N],
        (x, _) if x == width => vec![Direction::W],
        _ => unreachable!()
    }
}

fn energize(position: &Cell, dir: Direction, matrix: &Vec<Vec<Cell>>, energized_cells: &mut HashMap<Coord, Direction>) {
    if let Some(visited_cell) = energized_cells.get(&position.coord) {
        if *visited_cell == dir {
            return
        } else {
            energized_cells.insert(position.coord, dir)
        }
    } else {
        energized_cells.insert(position.coord, dir)
    };
    let dirs: Vec<Direction> = match position.symbol {
        Symbol::Forward => match dir {
            Direction::N => vec![Direction::E],
            Direction::E => vec![Direction::N],
            Direction::S => vec![Direction::W],
            Direction::W => vec![Direction::S],
        },
        Symbol::Back => match dir {
            Direction::N => vec![Direction::W],
            Direction::E => vec![Direction::S],
            Direction::S => vec![Direction::E],
            Direction::W => vec![Direction::N],
        },
        Symbol::Horizontal => match dir {
            Direction::N | Direction::S => vec![Direction::E, Direction::W],
            _ => vec![dir],
        },
        Symbol::Vertical => match dir {
            Direction::E | Direction::W => vec![Direction::N, Direction::S],
            _ => vec![dir],
        },

        Symbol::Empty => vec![dir],
    };

    for dir in dirs {
        if let Some(cell) = position.next_cell(dir, matrix) {
            energize(cell, dir, matrix, energized_cells);
        }
    }
}

struct Cell {
    symbol: Symbol,
    coord: Coord,
}

impl Cell {
    fn new(c: char, x: usize, y: usize) -> Self {
        Cell {
            symbol: c.into(),
            coord: Coord { x, y },
        }
    }

    fn next_cell<'a>(&'a self, direction: Direction, matrix: &'a Vec<Vec<Cell>>) -> Option<&Cell> {
        match direction {
            Direction::N => {
                let y = self.coord.y.checked_sub(1);
                let x = self.coord.x;
                y.and_then(|y| {
                    matrix.get(y).and_then(|row| row.get(x))
                })
            }
            Direction::E => {
                let y = self.coord.y;
                let x = self.coord.x.checked_add(1);
                x.and_then(|x| {
                    matrix.get(y).and_then(|row| row.get(x))
                })
            }
            Direction::S => {
                let y = self.coord.y.checked_add(1);
                let x = self.coord.x;
                y.and_then(|y| {
                    matrix.get(y).and_then(|row| row.get(x))
                })
            }
            Direction::W => {
                let y = self.coord.y;
                let x = self.coord.x.checked_sub(1);
                x.and_then(|x| {
                    matrix.get(y).and_then(|row| row.get(x))
                })
            }
        }
    }
}

enum Symbol {
    Empty,
    Vertical,
    Horizontal,
    Forward,
    Back,
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Symbol::Empty,
            '/' => Symbol::Forward,
            '\\' => Symbol::Back,
            '-' => Symbol::Horizontal,
            '|' => Symbol::Vertical,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part1(data), 46)
    }

    #[test]
    fn part2_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part2(data), 51)
    }
}

