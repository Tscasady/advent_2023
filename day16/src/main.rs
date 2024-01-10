use std::collections::HashMap;

fn main() {
    let data = include_str!("../input.txt");
    println!("{}", part1(data));
    println!("{}", part2(data));
}

fn part1(data: &str) -> usize {
    let matrix: Matrix = Matrix {
        data: data
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, symbol)| Cell::new(symbol, col, row))
                    .collect()
            })
            .collect(),
    };

    let mut beam = matrix.energize(Direction::E, Coord { x: 0, y: 0 });
    beam.calc_energy()
}

fn part2(data: &str) -> usize {
    let matrix: Matrix = Matrix {
        data: data
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, symbol)| Cell::new(symbol, col, row))
                    .collect()
            })
            .collect(),
    };

    let mut results: Vec<usize> = vec![];
    for (row_index, _row) in matrix.data.iter().enumerate() {
        for (col_index, _col) in matrix.data.iter().enumerate() {
            if row_index == 0
                || row_index == matrix.data.len() - 1
                || col_index == 0
                || col_index == matrix.data[0].len() - 1
            {
                let dirs = get_start_dirs(col_index, row_index, &matrix);
                for dir in dirs {
                    let mut beam = matrix.energize(dir, Coord { x: col_index, y: row_index});
                    results.push(beam.calc_energy());
                }
            }
        }
    }
    *results.iter().max().unwrap()
}

fn get_start_dirs(x: usize, y: usize, matrix: &Matrix) -> Vec<Direction> {
    let width = matrix.data[0].len() - 1;
    let height = matrix.data.len() - 1;

    match (x, y) {
        (0, 0) => vec![Direction::E, Direction::S],
        (0, y) if y == height => vec![Direction::N, Direction::E],
        (x, 0) if x == width => vec![Direction::W, Direction::S],
        (x, y) if x == width && y == height => vec![Direction::W, Direction::N],
        (0, _) => vec![Direction::E],
        (_, 0) => vec![Direction::S],
        (_, y) if y == height => vec![Direction::N],
        (x, _) if x == width => vec![Direction::W],
        _ => unreachable!(),
    }
}

struct Matrix {
    data: Vec<Vec<Cell>>,
}

impl Matrix {
    fn energize(&self, direction: Direction, start: Coord) -> Beam {
        Beam {
            cell: Some(&self.data[start.y][start.x]),
            direction,
            prev_direction: direction,
            matrix: self,
            queue: vec![],
            energized_cells: HashMap::default(),
        }
    }
    fn get(&self, row: usize) -> Option<&Vec<Cell>> {
        self.data.get(row)
    }
}

struct Beam<'a> {
    cell: Option<&'a Cell>,
    direction: Direction,
    prev_direction: Direction,
    matrix: &'a Matrix,
    queue: Vec<(Coord, Direction)>,
    energized_cells: HashMap<Coord, Vec<Direction>>,
}

impl<'a> Beam<'a> {
    fn get_next_dir(&mut self) -> Direction {
        match self.cell.unwrap().symbol {
            Symbol::Forward => match self.direction {
                Direction::N => Direction::E,
                Direction::E => Direction::N,
                Direction::S => Direction::W,
                Direction::W => Direction::S,
            },
            Symbol::Back => match self.direction {
                Direction::N => Direction::W,
                Direction::E => Direction::S,
                Direction::S => Direction::E,
                Direction::W => Direction::N,
            },
            Symbol::Horizontal => match self.direction {
                Direction::N | Direction::S => {
                    self.queue
                        .push((self.cell.unwrap().coord, Direction::E));
                    self.prev_direction = Direction::W;
                    Direction::W
                }
                _ => self.direction,
            },
            Symbol::Vertical => match self.direction {
                Direction::E | Direction::W => {
                    self.queue
                        .push((self.cell.unwrap().coord, Direction::N));
                    self.prev_direction = Direction::S;
                    Direction::S
                }
                _ => self.direction,
            },
            Symbol::Empty => self.direction,
        }
    }

    fn calc_energy(&mut self) -> usize {
        let mut looped = false;
        //energize our starting cell
        self.cell.unwrap().energize_cell(self.direction, &mut self.energized_cells);
        while let Some(cell) = self.next(looped) {
            looped = cell.energize_cell(self.direction, &mut self.energized_cells);
        }
        self.energized_cells.len()
    }

    fn next(&mut self, looped: bool) -> Option<&'a Cell> {
        if let Some(cell) = self.cell {
                if !looped {
                    self.direction = self.get_next_dir();
                    //try to get the next cell until it fails and queue is empty
                    self.cell = cell.next_cell(self.direction, self.matrix).or_else(|| {
                        //if none try the queue
                        self.next_from_queue()
                    });
                } else {
                    self.cell = self.next_from_queue();
                };
        }
        self.cell
    }

    fn next_from_queue(&mut self) -> Option<&'a Cell> {
        while !self.queue.is_empty() {
            if let Some((coord, dir)) = self.queue.pop() {
                self.direction = dir;
                match self.matrix.data[coord.y][coord.x]
                    .next_cell(self.direction, self.matrix)
                {
                    Some(cell) => return Some(cell),
                    None => continue,
                }
            }
        }
        None
    }
}

#[derive(Debug)]
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

    //this shoudl belong to beam
    fn next_cell<'a>(&'a self, direction: Direction, matrix: &'a Matrix) -> Option<&Cell> {
        match direction {
            Direction::N => {
                let y = self.coord.y.checked_sub(1);
                let x = self.coord.x;
                y.and_then(|y| matrix.get(y).and_then(|row| row.get(x)))
            }
            Direction::E => {
                let y = self.coord.y;
                let x = self.coord.x.checked_add(1);
                x.and_then(|x| matrix.get(y).and_then(|row| row.get(x)))
            }
            Direction::S => {
                let y = self.coord.y.checked_add(1);
                let x = self.coord.x;
                y.and_then(|y| matrix.get(y).and_then(|row| row.get(x)))
            }
            Direction::W => {
                let y = self.coord.y;
                let x = self.coord.x.checked_sub(1);
                x.and_then(|x| matrix.get(y).and_then(|row| row.get(x)))
            }
        }
    }

    fn energize_cell(
        &self,
        dir: Direction,
        energized_cells: &mut HashMap<Coord, Vec<Direction>>,
    ) -> bool {
        if let Some(visited_cell) = energized_cells.get(&self.coord) {
            if visited_cell.contains(&dir) {
                true
            } else {
                energized_cells
                    .entry(self.coord)
                    .and_modify(|list| list.push(dir));
                false
            }
        } else {
            energized_cells.insert(self.coord, vec![dir]);
            false
        }
    }
}

#[derive(Debug)]
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
