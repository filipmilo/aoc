use std::fs;

pub fn solve_part_1() -> i32 {
    fs::read_to_string("src/year_2023/inputs/day_1.txt")
        .expect("Oops, no file found.")
        .lines()
        .map(|line| {
            line.to_string()
                .chars()
                .filter(|ch| ch.is_numeric())
                .fold("".to_string(), |mut acc, curr| {
                    if acc.len() > 1 {
                        acc.pop();
                    }
                    if acc.len() == 0 {
                        acc.push_str(format!("{curr}{curr}").as_str());
                    } else {
                        acc.push(curr);
                    }

                    acc
                })
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>()
}

pub fn solve_part_2() -> i32 {
    let mappings = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    fs::read_to_string("src/year_2023/inputs/day_1.txt")
        .expect("Oops, no file found.")
        .lines()
        .map(|line| {
            line.to_string()
                .chars()
                .fold(("".to_string(), "".to_string()), |mut acc, curr| {
                    let num = if curr.is_numeric() {
                        (curr, true)
                    } else {
                        acc.1.push(curr);

                        (
                            if acc.1.len() > 5 {
                                let found = acc
                                    .1
                                    .chars()
                                    .collect::<Vec<char>>()
                                    .windows(5)
                                    .filter_map(|win| {
                                        String::from(
                                            match mappings.iter().find(|map| {
                                                win.iter().collect::<String>().contains(map.0)
                                            }) {
                                                Some(val) => val.1,
                                                None => '-',
                                            },
                                        )
                                        .parse::<i32>()
                                        .ok()
                                    })
                                    .last();

                                match found {
                                    Some(val) => char::from_digit(val as u32, 10).unwrap(),
                                    None => '-',
                                }
                            } else {
                                match mappings.iter().find(|map| acc.1.contains(map.0)) {
                                    Some(val) => val.1,
                                    None => '-',
                                }
                            },
                            false,
                        )
                    };

                    if num.0 != '-' {
                        if num.1 {
                            acc.1.clear();
                        }

                        if acc.0.len() > 1 {
                            acc.0.pop();
                        }

                        if acc.0.len() == 0 {
                            acc.0.push_str(format!("{}{}", num.0, num.0).as_str());
                        } else {
                            acc.0.push(num.0);
                        }
                    }

                    acc
                })
                .0
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>()
}
