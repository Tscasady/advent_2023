use std::collections::HashMap;

fn main() {
    let data = include_str!("../input.txt");
    println!("{}", part1(data))
}

fn part1(data: &str) -> i32 {
    let mut y: i32 = 0;
    let mut x: i32 = 0;
    let mut matrix: HashMap<i32, Vec<Edge>> = HashMap::new();
    for line in data.lines() {
        let edges = Edge::new(line, &mut x, &mut y);
        // println!("{:?}", edges);
        x = edges.last().unwrap().coord.x;
        y = edges.last().unwrap().coord.y;
        for edge in edges {
            matrix.entry(edge.coord.y).or_default().push(edge)
        }
    }

    // println!("{:?}", matrix);

    let area = matrix
        .values_mut()
        .map(|v| {
            v.sort_by(|a, b| a.coord.x.cmp(&b.coord.x));
            println!("{:?}", v);
            if v.len() % 2 == 0 {
                v.chunks(2)
                    .map(|chunk| (chunk[0].coord.x - chunk[1].coord.x).abs() + 1)
                    .sum::<i32>()
            } else {
                (v.first().unwrap().coord.x - v.last().unwrap().coord.x).abs() + 1
            }
        })
        .sum();

    area
}

#[derive(Debug)]
struct Edge {
    coord: Coord,
    color: String,
}

impl Edge {
    fn new(data: &str, x: &mut i32, y: &mut i32) -> Vec<Self> {
        let data: Vec<&str> = data.split_whitespace().collect();
        let length: i32 = data[1].parse().unwrap();
        let color = data[2];
        match Into::<Direction>::into(data[0]) {
            Direction::N => (*y + 1..=(*y + length))
                .map(|y| Edge {
                    coord: Coord { x: *x, y },
                    color: color.to_string(),
                })
                .collect(),
            Direction::S => (*y - length..=*y - 1).rev()
                .map(|y| Edge {
                    coord: Coord { x: *x, y },
                    color: color.to_string(),
                })
                .collect(),
            Direction::E => {
                vec![Edge {
                    coord: Coord {
                        x: *x + length,
                        y: *y,
                    },
                    color: color.to_string(),
                }]
            }
            Direction::W => {
                vec![Edge {
                    coord: Coord {
                        x: *x - length,
                        y: *y,
                    },
                    color: color.to_string(),
                }]
            }
        }
    }
}

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

enum Direction {
    N,
    S,
    E,
    W,
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "L" => Direction::W,
            "R" => Direction::E,
            "U" => Direction::N,
            "D" => Direction::S,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part1(data), 62)
    }
}
