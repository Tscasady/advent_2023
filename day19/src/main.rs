use core::fmt;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn main() {
    let data = include_str!("../input.txt");
    println!("{}", part1(data));
    println!("{}", part2(data));
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
        .fold(0, |acc, part| acc + part.sum())
}

fn part2(data: &str) -> usize {
    let data: Vec<&str> = data.split("\n\n").collect();
    let mut workflows: HashMap<String, Workflow> = Default::default();
    for line in data[0].lines() {
        let workflow = Workflow::new(line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    let mut range_results = RangeResults {
        x: HashSet::new(),
        m: HashSet::new(),
        a: HashSet::new(),
        s: HashSet::new(),
    };
    let mut queue: Vec<(PartRange, String)> = Vec::new();
    queue.push((PartRange::new(), "in".to_string()));

    //send a range through wokflow,

    let mut count = 0;
    while let Some((mut range, result)) = queue.pop() {
        println!("{:?}", range);
        println!("{:?}", range_results);
        range.eval(&workflows, result, &mut queue, &mut range_results);
        count += 1;
        if count > 11 {
            break;
        }
    }

    range_results.total()
}

#[derive(PartialEq, Eq, Clone)]
struct PartRange {
    x: Vec<i32>,
    m: Vec<i32>,
    a: Vec<i32>,
    s: Vec<i32>,
}

impl fmt::Debug for PartRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PartRange {{ x: {:?}-{:?}, m: {:?}-{:?}, a: {:?}-{:?}, s: {:?}-{:?},}}",
            self.x.iter().min(),
            self.x.iter().max(),
            self.m.iter().min(),
            self.m.iter().max(),
            self.a.iter().min(),
            self.a.iter().max(),
            self.s.iter().min(),
            self.s.iter().max()
        )
    }
}

struct RangeResults {
    x: HashSet<i32>,
    m: HashSet<i32>,
    a: HashSet<i32>,
    s: HashSet<i32>,
}

impl fmt::Debug for RangeResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RangeResults {{ x: {:?}, m: {:?}, a: {:?}, s: {:?}}}",
            self.find_ranges(Category::X),
            self.find_ranges(Category::M),
            self.find_ranges(Category::A),
            self.find_ranges(Category::S),
        )
    }
}

impl RangeResults {
    fn total(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    fn find_ranges(&self, category: Category) -> Vec<(i32, i32)> {
        let mut ranges = Vec::new();
        let mut start = None;
        let mut end = None;

        match category {
            Category::X => {
                let mut sorted_numbers: Vec<_> = self.x.iter().copied().collect();
                sorted_numbers.sort();
                for &num in sorted_numbers.iter() {
                    match end {
                        Some(prev) if prev + 1 == num => {
                            end = Some(num);
                        }
                        Some(_) => {
                            ranges.push((start.unwrap(), end.unwrap()));
                            start = Some(num);
                            end = Some(num);
                        }
                        None => {
                            start = Some(num);
                            end = Some(num);
                        }
                    }
                }

                if let Some(start) = start {
                    ranges.push((start, end.unwrap() + 1));
                }

                ranges
            }
            Category::M => {
                let mut sorted_numbers: Vec<_> = self.m.iter().copied().collect();
                sorted_numbers.sort();
                for &num in sorted_numbers.iter() {
                    match end {
                        Some(prev) if prev + 1 == num => {
                            end = Some(num);
                        }
                        Some(prev) => {
                            ranges.push((start.unwrap(), prev + 1));
                            start = Some(num);
                            end = Some(num);
                        }
                        None => {
                            start = Some(num);
                            end = Some(num);
                        }
                    }
                }

                if let Some(start) = start {
                    ranges.push((start, end.unwrap() + 1));
                }

                ranges
            }
            Category::A => {
                let mut sorted_numbers: Vec<_> = self.a.iter().copied().collect();
                sorted_numbers.sort();
                for &num in sorted_numbers.iter() {
                    match end {
                        Some(prev) if prev + 1 == num => {
                            end = Some(num);
                        }
                        Some(prev) => {
                            ranges.push((start.unwrap(), prev + 1));
                            start = Some(num);
                            end = Some(num);
                        }
                        None => {
                            start = Some(num);
                            end = Some(num);
                        }
                    }
                }

                if let Some(start) = start {
                    ranges.push((start, end.unwrap() + 1));
                }

                ranges
            }
            Category::S => {
                let mut sorted_numbers: Vec<_> = self.s.iter().copied().collect();
                sorted_numbers.sort();
                for &num in sorted_numbers.iter() {
                    match end {
                        Some(prev) if prev + 1 == num => {
                            end = Some(num);
                        }
                        Some(prev) => {
                            ranges.push((start.unwrap(), prev + 1));
                            start = Some(num);
                            end = Some(num);
                        }
                        None => {
                            start = Some(num);
                            end = Some(num);
                        }
                    }
                }

                if let Some(start) = start {
                    ranges.push((start, end.unwrap() + 1));
                }

                ranges
            }
        }
    }
}

impl PartRange {
    fn new() -> Self {
        PartRange {
            x: (1..=4000).collect(),
            m: (1..=4000).collect(),
            a: (1..=4000).collect(),
            s: (1..=4000).collect(),
        }
    }

    fn eval(
        &mut self,
        workflows: &HashMap<String, Workflow>,
        result: String,
        queue: &mut Vec<(PartRange, String)>,
        range_results: &mut RangeResults,
    ) {
        let mut result = result.clone();
        'outer: while let Some(workflow) = workflows.get(&result) {
            for test in workflow.tests.iter() {
                if self.test(test) {
                    match test.comparison {
                        Ordering::Less => {
                            match test.category {
                                Category::X => {
                                    queue.push((
                                        PartRange {
                                            x: self
                                                .x
                                                .iter()
                                                .cloned()
                                                .filter(|num| *num >= test.value)
                                                .collect(),
                                            m: self.m.clone(),
                                            a: self.a.clone(),
                                            s: self.s.clone(),
                                        },
                                        workflow.name.clone(),
                                    ));
                                    self.x.retain(|num| num < &test.value);
                                }
                                Category::M => {
                                    queue.push((
                                        PartRange {
                                            m: self
                                                .m
                                                .iter()
                                                .cloned()
                                                .filter(|num| *num >= test.value)
                                                .collect(),
                                            a: self.a.clone(),
                                            s: self.s.clone(),
                                            x: self.x.clone(),
                                        },
                                        workflow.name.clone(),
                                    ));
                                    self.m.retain(|num| num < &test.value);
                                }
                                Category::A => {
                                    queue.push((
                                        PartRange {
                                            a: self
                                                .a
                                                .iter()
                                                .cloned()
                                                .filter(|num| *num >= test.value)
                                                .collect(),
                                            s: self.s.clone(),
                                            x: self.x.clone(),
                                            m: self.m.clone(),
                                        },
                                        workflow.name.clone(),
                                    ));
                                    self.a.retain(|num| num < &test.value);
                                }
                                Category::S => {
                                    queue.push((
                                        PartRange {
                                            s: self
                                                .s
                                                .iter()
                                                .cloned()
                                                .filter(|num| *num >= test.value)
                                                .collect(),
                                            x: self.x.clone(),
                                            m: self.m.clone(),
                                            a: self.a.clone(),
                                        },
                                        workflow.name.clone(),
                                    ));
                                    self.s.retain(|num| num < &test.value);
                                }
                            };
                        }
                        Ordering::Greater => match test.category {
                            Category::X => {
                                queue.push((
                                    PartRange {
                                        x: self
                                            .x
                                            .iter()
                                            .cloned()
                                            .filter(|num| *num <= test.value)
                                            .collect(),
                                        m: self.m.clone(),
                                        a: self.a.clone(),
                                        s: self.s.clone(),
                                    },
                                    workflow.name.clone(),
                                ));
                                self.x.retain(|num| num > &test.value);
                            }
                            Category::M => {
                                queue.push((
                                    PartRange {
                                        m: self
                                            .m
                                            .iter()
                                            .cloned()
                                            .filter(|num| *num <= test.value)
                                            .collect(),
                                        a: self.a.clone(),
                                        s: self.s.clone(),
                                        x: self.x.clone(),
                                    },
                                    workflow.name.clone(),
                                ));
                                self.m.retain(|num| num > &test.value);
                            }
                            Category::A => {
                                queue.push((
                                    PartRange {
                                        a: self
                                            .a
                                            .iter()
                                            .cloned()
                                            .filter(|num| *num <= test.value)
                                            .collect(),
                                        s: self.s.clone(),
                                        x: self.x.clone(),
                                        m: self.m.clone(),
                                    },
                                    workflow.name.clone(),
                                ));
                                self.a.retain(|num| num > &test.value);
                            }
                            Category::S => {
                                queue.push((
                                    PartRange {
                                        s: self
                                            .s
                                            .iter()
                                            .cloned()
                                            .filter(|num| *num <= test.value)
                                            .collect(),
                                        x: self.x.clone(),
                                        m: self.m.clone(),
                                        a: self.a.clone(),
                                    },
                                    workflow.name.clone(),
                                ));
                                self.s.retain(|num| num > &test.value);
                            }
                        },
                        _ => (),
                    }
                    result = test.next_workflow.clone();
                    // println!("{:?}", self.x.iter().max());
                    // println!("{:?}", self.m.iter().max());
                    // println!("{:?}", self.a.iter().max());
                    // println!("{:?}", self.s.iter().max());
                    if result == "A" {
                        //add to result
                        println!("self before adding to result {:?}", self);
                        self.update(range_results);
                        break 'outer;
                    } else if result == "R" {
                        println!("hi from 'r'");
                        break 'outer;
                    } else {
                        continue 'outer;
                    }
                }
            }
            result = workflow.default.clone()
        }
    }

    fn test(&mut self, test: &Test) -> bool {
        match test.comparison {
            Ordering::Less => self
                .category(test.category)
                .iter()
                .any(|num| num < &test.value),
            Ordering::Greater => self
                .category(test.category)
                .iter()
                .any(|num| num > &test.value),
            _ => unreachable!(),
        }
    }

    fn category(&mut self, category: Category) -> &mut Vec<i32> {
        match category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }

    fn update(&self, other: &mut RangeResults) {
        other.x.extend(&self.x);
        other.m.extend(&self.m);
        other.a.extend(&self.a);
        other.s.extend(&self.s);
    }
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

    #[test]
    fn part2_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part2(data), 167409079868000)
    }
}
