use std::collections::HashMap;

fn main() {
    let data: Vec<&str> = include_str!("../input.txt").lines().collect();
    let directions = data[0];
    let mut location = "AAA".to_string();
    let target = "ZZZ".to_string();
    let nodes: Vec<Vec<String>> = data[2..].iter().map(|line| {
        line.chars()
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<char>>()
            .chunks(3).map(|chunk| chunk.iter().collect::<String>()).collect::<Vec<String>>()
    }).collect();

    let mut map: HashMap<String, (String, String)> = Default::default(); 
    for node in nodes {
        map.insert(node[0].clone(), (node[1].clone(), node[2].clone()));
    }

    let mut count = 0;
    'outer: loop {
        let mut direction_iter = directions.chars(); 
        while let Some(direction) = direction_iter.next() {
                if direction == 'L' {
                    count += 1;
                    let next = map.get(&location).unwrap().0.clone();
                    if next == target {
                        break 'outer
                    }
                    location = next; 
                } else {
                    count += 1;
                    let next = map.get(&location).unwrap().1.clone();
                    if next == target {
                        break 'outer
                    }
                    location = next; 
                }
            }
        };
    println!("{}", count)
}
