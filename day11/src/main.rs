fn main() {
    let content = include_str!("../input.txt");
    println!("{}", part1(content));
    println!("{}", part2(content, 1000000));
}

fn part1(data: &str) -> usize {
    let universe = parse_input(data);
    let mut expanded_universe = universe.clone();
    let mut cols = vec![true; universe[0].len()];
    let mut offset = 0;
    for (row_index, row) in universe.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            expanded_universe.insert(row_index + offset, vec!['.'; row.len()]);
            offset += 1;
        }
        for (col_index, c) in row.iter().enumerate() {
            if *c != '.' {
                cols[col_index] = false
            }
        }
    }

    offset = 0;
    for (index, col) in cols.iter().enumerate() {
        if *col {
            insert_column(&mut expanded_universe, index + offset);
            offset += 1;
        }
    }

    let mut galaxies: Vec<Galaxy> = vec![];

    for (y, row) in expanded_universe.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem != '.' {
                galaxies.push(Galaxy { original_x: x, original_y: y, x, y })
            }
        }
    }

    let mut result = 0;
    while let Some(current_galaxy) = galaxies.pop() {
        result += galaxies.iter().fold(0, |acc, galaxy| {
            let distance = galaxy.x.abs_diff(current_galaxy.x) + galaxy.y.abs_diff(current_galaxy.y);
            acc + distance 
        });
    }

    result
}

fn part2(data: &str, expansion_rate: usize) -> usize {
    let universe = parse_input(data);
    let mut galaxies: Vec<Galaxy> = vec![];
    let mut cols = vec![true; universe[0].len()];
    
    for (y, row) in universe.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem != '.' {
                galaxies.push(Galaxy { original_x: x, x, original_y: y, y })
            }
        }
    }

    // println!("{:?}", galaxies);
    
    for (row_index, row) in universe.iter().enumerate() {
        if row.iter().all(|c| *c == '.') {
            galaxies.iter_mut().filter(|galaxy| galaxy.original_y > row_index).for_each(|galaxy| galaxy.y += expansion_rate - 1);
        }
        for (col_index, c) in row.iter().enumerate() {
            if *c != '.' {
                cols[col_index] = false
            }
        }
    }

    // println!("Row modified: {:?}", galaxies);

    for (index, col) in cols.iter().enumerate() {
        if *col {
            galaxies.iter_mut().filter(|galaxy| galaxy.original_x > index).for_each(|galaxy| galaxy.x += expansion_rate - 1)
        }
    }

    for galaxy in galaxies.iter() {
        println!("{:?}", galaxy);
    }

    let mut result = 0;
    while let Some(current_galaxy) = galaxies.pop() {
        result += galaxies.iter().fold(0, |acc, galaxy| {
            let distance = galaxy.x.abs_diff(current_galaxy.x) + galaxy.y.abs_diff(current_galaxy.y);
            acc + distance 
        });
    }

    result

}
fn parse_input(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|lines| lines.chars().collect()).collect()
}

fn insert_column(matrix: &mut [Vec<char>], position: usize) {
    for row in matrix.iter_mut() {
        row.insert(position, '.');
    }
}


#[derive(Debug)]
struct Galaxy {
    original_x: usize,
    original_y: usize,
    x: usize,
    y: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part1(data), 374)
    }

    #[test]
    fn part2_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part2(data, 10), 1030);

        let data = include_str!("./test.txt");
        assert_eq!(part2(data, 100), 8410)
    }
}
