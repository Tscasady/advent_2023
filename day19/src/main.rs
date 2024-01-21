use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::RangeInclusive;

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

fn part2(data: &str) -> i64 {
    let data: Vec<&str> = data.split("\n\n").collect();
    let mut workflows: HashMap<String, Workflow> = Default::default();
    for line in data[0].lines() {
        let workflow = Workflow::new(line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    let mut total: i64 = 0;
    let mut ranges: Vec<PartRange> = vec![];
    let mut queue: Vec<(PartRange, String)> = Vec::new();
    queue.push((PartRange::new(), "in".to_string()));

    //send a range through wokflow,

    while let Some((mut range, result)) = queue.pop() {
        range.eval(&workflows, result, &mut queue, &mut ranges, &mut total);
    }

    total
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct PartRange {
    x: RangeInclusive<i32>,
    m: RangeInclusive<i32>,
    a: RangeInclusive<i32>,
    s: RangeInclusive<i32>,
}

impl PartRange {
    fn new() -> Self {
        PartRange {
            x: (1..=4000),
            m: (1..=4000),
            a: (1..=4000),
            s: (1..=4000),
        }
    }

    fn eval(
        &mut self,
        workflows: &HashMap<String, Workflow>,
        result: String,
        queue: &mut Vec<(PartRange, String)>,
        ranges: &mut Vec<PartRange>,
        total: &mut i64,
    ) {
        let mut result = result.clone();
        'outer: while let Some(workflow) = workflows.get(&result) {
            for test in workflow.tests.iter() {
                if self.test(test) {
                    match test.comparison {
                        Ordering::Less => {
                            match test.category {
                                Category::X => {
                                    if self.x.end() >= &test.value {
                                        queue.push((
                                            PartRange {
                                                x: (test.value..=*self.x.end()),
                                                m: self.m.clone(),
                                                a: self.a.clone(),
                                                s: self.s.clone(),
                                            },
                                            workflow.name.clone(),
                                        ));
                                    }
                                    self.x = *self.x.start()..=(min(test.value - 1, *self.x.end()));
                                }
                                Category::M => {
                                    if self.m.end() >= &test.value {
                                        queue.push((
                                            PartRange {
                                                m: (test.value..=*self.m.end()),
                                                a: self.a.clone(),
                                                s: self.s.clone(),
                                                x: self.x.clone(),
                                            },
                                            workflow.name.clone(),
                                        ));
                                    }
                                    self.m =
                                        (*self.m.start()..=(min(test.value - 1, *self.m.end())));
                                }
                                Category::A => {
                                    if self.a.end() >= &test.value {
                                        queue.push((
                                            PartRange {
                                                a: (test.value..=*self.a.end()),
                                                s: self.s.clone(),
                                                x: self.x.clone(),
                                                m: self.m.clone(),
                                            },
                                            workflow.name.clone(),
                                        ));
                                    }
                                    self.a =
                                        (*self.a.start()..=(min(test.value - 1, *self.a.end())));
                                }
                                Category::S => {
                                    if self.s.end() >= &test.value {
                                        queue.push((
                                            PartRange {
                                                s: (test.value..=*self.s.end()),
                                                x: self.x.clone(),
                                                m: self.m.clone(),
                                                a: self.a.clone(),
                                            },
                                            workflow.name.clone(),
                                        ));
                                    }
                                    self.s = *self.s.start()..=(min(test.value - 1, *self.s.end()));
                                }
                            };
                        }
                        Ordering::Greater => match test.category {
                            Category::X => {
                                if self.x.start() <= &test.value {
                                    queue.push((
                                        PartRange {
                                            x: (*self.x.start()..=test.value),
                                            m: self.m.clone(),
                                            a: self.a.clone(),
                                            s: self.s.clone(),
                                        },
                                        workflow.name.clone(),
                                    ));
                                }
                                self.x = (max(test.value + 1, *self.x.start())..=*self.x.end());
                            }
                            Category::M => {
                                if self.m.start() <= &test.value {
                                    queue.push((
                                        PartRange {
                                            m: (*self.m.start()..=test.value),
                                            a: self.a.clone(),
                                            s: self.s.clone(),
                                            x: self.x.clone(),
                                        },
                                        workflow.name.clone(),
                                    ));
                                }
                                self.m = (max(test.value + 1, *self.m.start())..=*self.m.end());
                            }
                            Category::A => {
                                if self.a.start() <= &test.value {
                                    queue.push((
                                        PartRange {
                                            a: (*self.a.start()..=test.value),
                                            s: self.s.clone(),
                                            x: self.x.clone(),
                                            m: self.m.clone(),
                                        },
                                        workflow.name.clone(),
                                    ));
                                }
                                self.a = (max(test.value + 1, *self.a.start())..=*self.a.end());
                            }
                            Category::S => {
                                if self.s.start() <= &test.value {
                                    queue.push((
                                        PartRange {
                                            s: (*self.s.start()..=test.value),
                                            x: self.x.clone(),
                                            m: self.m.clone(),
                                            a: self.a.clone(),
                                        },
                                        workflow.name.clone(),
                                    ));
                                }
                                self.s = (max(test.value + 1, *self.s.start())..=*self.s.end());
                            }
                        },
                        _ => (),
                    }
                    result = test.next_workflow.clone();
                    if result == "A" {
                        *total += self.total(ranges);
                        ranges.push(self.clone());

                        break 'outer;
                    } else if result == "R" {
                        break 'outer;
                    } else {
                        continue 'outer;
                    }
                }
            }
            result = workflow.default.clone();
            if result == "A" {
                *total += self.total(ranges);
                ranges.push(self.clone());

                break 'outer;
            } else if result == "R" {
                break 'outer;
            } else {
                continue 'outer;
            }
        }
    }

    fn test(&mut self, test: &Test) -> bool {
        match test.comparison {
            Ordering::Less => {
                let range = self.category(test.category);
                range.start() < &test.value
            }

            Ordering::Greater => {
                let range = self.category(test.category);
                range.end() > &test.value
            }
            _ => unreachable!(),
        }
    }

    fn category(&mut self, category: Category) -> &RangeInclusive<i32> {
        match category {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }

    fn total(&self, all_ranges: &[PartRange]) -> i64 {
        let duplicates = all_ranges
            .iter()
            .fold(0, |acc, range| acc + self.find_diff(range));
        max(0, self.possible() - duplicates)
    }

    fn possible(&self) -> i64 {
        len(&self.x) * len(&self.m) * len(&self.a) * len(&self.s)
    }

    fn find_diff(&self, range: &PartRange) -> i64 {
        calc_overlap(&self.x, &range.x)
            * calc_overlap(&self.m, &range.m)
            * calc_overlap(&self.a, &range.a)
            * calc_overlap(&self.s, &range.s)
    }
}

fn calc_overlap(range1: &RangeInclusive<i32>, range2: &RangeInclusive<i32>) -> i64 {
    if range1.start() <= range2.start() && range1.end() >= range2.end() {
        (range2.end() - range2.start() + 1).into()
    } else if range2.start() <= range1.start() && range2.end() >= range1.end() {
        (range1.end() - range1.start() + 1).into()
    } else if range2.start() < range1.start() {
        max(0, (range2.end() - range1.start() + 1).into())
    } else {
        max(0, (range1.end() - range2.start() + 1).into())
    }
}

fn len(field: &RangeInclusive<i32>) -> i64 {
    (field.end() - field.start() + 1).into()
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

    #[test]
    fn range_overlap_test() {
        let pr = PartRange {
            x: (1..=3),
            m: (2..=4),
            a: (1..=6),
            s: (1..=3),
        };
        let ranges = vec![
            PartRange {
                x: (2..=4),
                m: (2..=2),
                a: (2..=4),
                s: (1..=1),
            },
            PartRange {
                x: (1..=3),
                m: (2..=2),
                a: (2..=4),
                s: (1..=1),
            },
        ];
        assert_eq!(pr.total(&ranges), 147)
    }

    #[test]
    fn calc_overlap_test() {
        let r: RangeInclusive<i32> = (5..=10);
        let r2: RangeInclusive<i32> = (5..=7);
        assert_eq!(calc_overlap(&r, &r2), 3);

        let r: RangeInclusive<i32> = (1..=3);
        let r2: RangeInclusive<i32> = (1..=3);
        assert_eq!(calc_overlap(&r, &r2), 3);

        let r: RangeInclusive<i32> = (1..=2);
        let r2: RangeInclusive<i32> = (3..=4);
        assert_eq!(calc_overlap(&r, &r2), 0);
    }
}
