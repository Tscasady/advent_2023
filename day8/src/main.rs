use std::collections::HashMap;

fn main() {
    let data: Vec<&str> = include_str!("../input.txt").lines().collect();
    let directions = data[0];
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

    let locations = map.keys().filter(|key| key.ends_with('A')).cloned().collect::<Vec<String>>();
    let mut loop_sizes: Vec<u128> = vec![0; locations.len()];
    'outer: for (index, location) in locations.iter().enumerate() {
        let mut next_location: String = location.clone();
        let mut count: u128 = 0;
        let mut z_count: u8 = 0;
        let mut loop_start: u128 = 0;
        loop {
        let mut direction_iter = directions.chars(); 
        while let Some(direction) = direction_iter.next() {
                if direction == 'L' {
                    count += 1;
                    next_location = map.get(&next_location).unwrap().0.clone();
                    if next_location.ends_with('Z') {
                        z_count += 1;
                        if z_count > 1 {
                            loop_sizes[index] = count - loop_start;
                            continue 'outer
                        }
                        loop_start = count;
                    }
                } else {
                    count += 1;
                    next_location = map.get(&next_location).unwrap().1.clone();
                    if next_location.ends_with('Z') {
                        z_count += 1;
                        if z_count > 1 {
                            loop_sizes[index] = count - loop_start;
                            continue 'outer
                        }
                        loop_start = count;
                    }
                }
            }
        }
    };
    println!("{:?}", math::lcm_of_set(&loop_sizes))
}

pub mod math {
    fn euclidean_gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    pub fn lcm(a: u128, b: u128) -> u128 {
        if a == 0 || b == 0 {
            0
        } else {
            (a / euclidean_gcd(a, b)) * b
        }
    }

    pub fn lcm_of_set(numbers: &[u128]) -> u128 {
        let mut current_lcm = numbers[0];

        for &num in &numbers[1..] {
            current_lcm = lcm(current_lcm, num);
        }

        current_lcm
    }

}
//get all 'a' into a vec
//set mut the vec with the values you moved to
// check for all 'z'
// repeat

//multithread solution
// for each starting location create a thread
//stop on z, report number of steps
//tell any thread that took less number of steps to take that many steps 
//check for all z
//repeat


//check how often i loop

