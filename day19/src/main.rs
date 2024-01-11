use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let data = include_str!("../input.txt");
    println!("{}", part1(data));
}

fn part1(data: &str) -> i32 {
    let data: Vec<&str> = data.split("\n\n").collect();
    let mut workflows: HashMap<String, Workflow> = Default::default();
    for line in data[0].lines() {
        let workflow = Workflow::new(line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    let parts: Vec<Part> = data[1].lines().map(Part::new).collect();
    parts
        .iter()
        .filter(|part| {
            let mut result = "in".to_string();
            let mut status = false;
            'outer: while let Some(workflow) = workflows.get(&result) {
                for test in workflow.tests.iter() {
                    if part.test(test) {
                        result = test.next_workflow.clone();
                        if result == "A" {
                            status = true;
                            break 'outer;
                        } else if result == "R" {
                            status = false;
                            break 'outer;
                        }
                        continue 'outer;
                    }
                }
                result = workflow.default.clone();
                if result == "A" {
                    status = true;
                    break 'outer;
                } else if result == "R" {
                    status = false;
                    break 'outer;
                }
            }
            status
        })
        .fold(0, |acc, part| {
            acc + part.sum()
        })
}

fn part2(data: &str) -> i32 {
    todo!()
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn new(data: &str) -> Self {
        let data: Vec<&str> = data
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split([',', '='])
            .collect();
        Part {
            x: data[1].parse().unwrap(),
            m: data[3].parse().unwrap(),
            a: data[5].parse().unwrap(),
            s: data[7].parse().unwrap(),
        }
    }

    fn test(&self, test: &Test) -> bool {
        match test.comparison {
            Ordering::Less => self.category(test.category) < test.value,
            Ordering::Greater => self.category(test.category) > test.value,
            _ => unreachable!(),
        }
    }

    fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }

    fn category(&self, category: Category) -> i32 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<char> for Category {
    fn from(c: char) -> Category {
        match c {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    tests: Vec<Test>,
    default: String,
}

impl Workflow {
    fn new(data: &str) -> Self {
        let data: Vec<&str> = data.trim_end_matches('}').split('{').collect();
        let test_data: Vec<&str> = data[1].split(',').collect();
        let mut tests: Vec<Test> = vec![];
        for test in test_data.iter().take(test_data.len() - 1) {
            tests.push(Test::new(test));
        }
        Workflow {
            name: data[0].to_string(),
            tests,
            default: test_data.last().unwrap().to_string(),
        }
    }
}

#[derive(Debug)]
struct Test {
    category: Category,
    comparison: Ordering,
    value: i32,
    next_workflow: String,
}

fn char_to_ordering(c: char) -> Ordering {
    match c {
        '<' => Ordering::Less,
        '>' => Ordering::Greater,
        _ => unreachable!(),
    }
}

impl Test {
    fn new(data: &str) -> Self {
        let test: Vec<&str> = data.split(':').collect();
        let mut chars = test[0].chars();

        let first = chars.next().unwrap();
        let category = first.into();
        let second = chars.next().unwrap();
        let comparison = char_to_ordering(second);
        let rest: String = chars.collect();

        Test {
            category,
            comparison,
            value: rest.parse().unwrap(),
            next_workflow: test.last().unwrap().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part1(data), 19114)
    }
}
