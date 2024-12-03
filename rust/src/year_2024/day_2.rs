use std::fs;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Asc,
    Desc,
}

impl Direction {
    fn from(curr: u32, prev: u32) -> Option<Direction> {
        if curr == prev {
            return None;
        }

        Some(if curr > prev {
            Direction::Asc
        } else {
            Direction::Desc
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Accumulate {
    prev: Option<u32>,
    direction: Option<Direction>,
    valid: bool,
}

impl Accumulate {
    fn from(acc: Accumulate, curr: u32) -> Accumulate {
        Accumulate {
            valid: match (acc.prev, acc.direction) {
                (Some(p), Some(d)) => {
                    p.abs_diff(curr) < 4
                        && p != curr
                        && Direction::from(curr, p).is_some_and(|v| v == d)
                }
                (Some(p), None) => {
                    p.abs_diff(curr) < 4 && p != curr && Direction::from(curr, p).is_some()
                }
                _ => acc.valid,
            },
            direction: if acc.prev.is_some() {
                Direction::from(curr, acc.prev.unwrap())
            } else {
                None
            },
            prev: Some(curr),
        }
    }
}

pub fn solve_part_1() -> u32 {
    fs::read_to_string("src/year_2024/inputs/day_2_d.txt")
        .expect("Oops, could not open file.")
        .lines()
        .filter(|l| {
            l.split_whitespace()
                .map(|l| l.parse::<u32>().unwrap())
                .fold(
                    Accumulate {
                        prev: None,
                        direction: None,
                        valid: true,
                    },
                    |acc, curr| {
                        if !acc.valid {
                            acc
                        } else {
                            Accumulate::from(acc, curr)
                        }
                    },
                )
                .valid
        })
        .count() as u32
}

pub fn solve_part_2() -> u32 {
    fs::read_to_string("src/year_2024/inputs/day_2.txt")
        .expect("Oops, could not open file.")
        .lines()
        .filter(|l| {
            let numbers = l
                .split_whitespace()
                .map(|l| l.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            numbers
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let mut vec = numbers.clone();
                    vec.remove(i);

                    vec.iter()
                        .fold(
                            Accumulate {
                                prev: None,
                                direction: None,
                                valid: true,
                            },
                            |acc, curr| {
                                if !acc.valid {
                                    acc
                                } else {
                                    Accumulate::from(acc, *curr)
                                }
                            },
                        )
                        .valid
                })
                .any(|v| v)
        })
        .count() as u32
}
