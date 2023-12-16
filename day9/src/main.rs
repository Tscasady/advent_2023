use core::str::Lines;

fn main() {
    let content = include_str!("../input.txt").lines();
    println!("{}", part1(content));
}

fn part1(data: Lines<'_>) -> i64 {
    let lines: Vec<Vec<i64>> = data.map(|line| {
        line.split(' ')
            .map(|num| num.parse::<i64>().unwrap())
            .collect()
    }).collect();
    lines.into_iter().map(|line| {
        let nums_diff = vec![];
        find_next_num(line, nums_diff)
            .iter()
            .fold(0, |acc, nums| acc + nums.last().unwrap())
    }).sum()

}


fn find_next_num(nums: Vec<i64>, mut nums_diff: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    nums_diff.push(nums.clone());
    let mut prev: Option<i64> = None;
    let mut next_nums: Vec<i64> = vec![]; 
    for num in &nums {
        if let Some(prev) = prev {
            next_nums.push(num - prev)
        } 
        prev = Some(*num)
    }

    if next_nums.iter().all(|num| *num == 0) {
        nums_diff
    } else {
        find_next_num(next_nums, nums_diff)
    }
}











#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("./test.txt").lines();
        assert_eq!(part1(input), 114)
    }

    #[test]
    fn find_next_test() {
        let input = vec![0, 3, 6];
        let result = vec![];
        assert_eq!(find_next_num(input, result), vec![vec![0, 3, 6], vec![3, 3]])
    }
}
