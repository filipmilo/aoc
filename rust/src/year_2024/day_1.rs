use std::{collections::HashMap, fs};

pub fn solve_part_1() -> i32 {
    let mut lists = fs::read_to_string("src/year_2024/inputs/day_1.txt")
        .expect("Oops, could not open file.")
        .lines()
        .fold((vec![], vec![]), |mut acc, curr| {
            let nums = curr.split_whitespace().collect::<Vec<&str>>();

            acc.0.push(nums[0].parse::<i32>().unwrap());
            acc.1.push(nums[1].parse::<i32>().unwrap());

            acc
        });

    lists.0.sort();
    lists.1.sort();

    lists
        .0
        .iter()
        .enumerate()
        .map(|(i, num)| (num - lists.1[i]).abs())
        .sum()
}

pub fn solve_part_2() -> i32 {
    let lists = fs::read_to_string("src/year_2024/inputs/day_1.txt")
        .expect("Oops, could not open file.")
        .lines()
        .fold((vec![], HashMap::new()), |mut acc, curr| {
            let nums = curr.split_whitespace().collect::<Vec<&str>>();

            acc.0.push(nums[0].parse::<i32>().unwrap());

            acc.1
                .entry(nums[1].parse::<i32>().unwrap())
                .and_modify(|n| *n += 1)
                .or_insert(1);

            acc
        });

    lists
        .0
        .iter()
        .map(|num| num * lists.1.get(num).unwrap_or(&0))
        .sum()
}
