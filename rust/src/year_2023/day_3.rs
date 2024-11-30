use std::collections::HashMap;
use std::fs;

struct Accumulate {
    curr_num: String,
    sum: u32,
}

struct AccumulatePart2 {
    curr_num: String,
    curr_num_start: Option<usize>,
    gears: Vec<usize>,
    numbers: HashMap<(usize, usize), u32>,
}

//140 -> one line size
pub fn _solve_part_1() -> u32 {
    let engine_map = fs::read_to_string("src/year_2023/inputs/day_3.txt")
        .expect("Oops, no file found.")
        .chars()
        .collect::<Vec<char>>();

    engine_map
        .iter()
        .enumerate()
        .fold(
            Accumulate {
                curr_num: "".to_string(),
                sum: 0,
            },
            |mut acc, curr| {
                if curr.1.is_numeric() {
                    acc.curr_num.push(*curr.1);
                } else if !acc.curr_num.is_empty() {
                    println!("{}", acc.curr_num);
                    let mut indicies: Vec<char> =
                        vec![*curr.1, engine_map[curr.0 - acc.curr_num.len() - 1]];

                    if curr.0 >= 140 {
                        let e = curr.0 - 141;
                        let b = e - (acc.curr_num.len() + 1);
                        indicies.extend(&engine_map[b..=e]);
                    }

                    if curr.0 <= engine_map.len() - 141 {
                        let e = curr.0 + 141;
                        let b = e - (acc.curr_num.len() + 1);
                        indicies.extend(&engine_map[b..=e]);
                    }

                    println!("{:?}", indicies);

                    if indicies
                        .iter()
                        .any(|s| *s != '.' && *s != '\n' && !s.is_numeric())
                    {
                        acc.sum += acc.curr_num.parse::<u32>().unwrap();
                    }

                    acc.curr_num.clear();
                }

                acc
            },
        )
        .sum
}

pub fn solve_part_2() -> u32 {
    let engine_map = fs::read_to_string("src/year_2023/inputs/day_3.txt")
        .expect("Oops, no file found.")
        .chars()
        .collect::<Vec<char>>();

    let mappings = engine_map.iter().enumerate().fold(
        AccumulatePart2 {
            curr_num: "".to_string(),
            curr_num_start: None,
            gears: vec![],
            numbers: HashMap::new(),
        },
        |mut acc, curr| {
            if curr.1.is_numeric() {
                acc.curr_num.push(*curr.1);

                if acc.curr_num_start.is_none() {
                    acc.curr_num_start = Some(curr.0);
                }

                return acc;
            }

            if !acc.curr_num.is_empty() {
                acc.numbers.insert(
                    (acc.curr_num_start.unwrap(), curr.0 - 1),
                    acc.curr_num.parse::<u32>().unwrap(),
                );
                acc.curr_num.clear();
                acc.curr_num_start = None;
            }

            if *curr.1 == '*' {
                acc.gears.push(curr.0);
            }

            acc
        },
    );

    mappings
        .gears
        .iter()
        .filter_map(|m| {
            let mut indicies = vec![*m - 1, *m + 1];

            if *m >= 140 {
                indicies.extend(&[*m - 140, *m - 141, *m - 142]);
            }

            if *m <= engine_map.len() - 141 {
                indicies.extend(&[*m + 140, *m + 141, *m + 142]);
            }

            let nums: Vec<&u32> = mappings
                .numbers
                .keys()
                .filter_map(|k| {
                    if indicies.iter().any(|i| k.0 <= *i && k.1 >= *i) {
                        Some(mappings.numbers.get(k).unwrap())
                    } else {
                        None
                    }
                })
                .collect();

            println!("{:?}", nums);
            println!("{:?}", indicies);

            if nums.len() == 2 {
                return Some(nums[0] * nums[1]);
            }

            None
        })
        .sum::<u32>()
}
