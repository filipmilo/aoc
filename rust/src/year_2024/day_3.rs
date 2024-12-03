use std::fs;

#[derive(Debug)]
enum State {
    Search(String),
    Num(Option<String>, Option<String>, bool),
}

impl State {
    fn change_state(&self, curr: char) -> State {
        match self {
            Self::Search(acc) => {
                let new = format!("{}{}", acc, curr);

                if new.ends_with("mul(") {
                    Self::Num(Some("".to_string()), None, false)
                } else {
                    Self::Search(new)
                }
            }
            Self::Num(num1, num2, _) => match (num1, num2) {
                (Some(v), None) => {
                    if curr.is_numeric() {
                        Self::Num(Some(format!("{}{}", v, curr)), None, false)
                    } else if curr == ',' {
                        Self::Num(Some(v.clone()), Some("".to_string()), false)
                    } else {
                        Self::Search(curr.to_string())
                    }
                }
                (v, Some(n)) => {
                    if curr.is_numeric() {
                        Self::Num(v.clone(), Some(format!("{}{}", n, curr)), false)
                    } else if curr == ')' {
                        Self::Num(v.clone(), Some(n.clone()), true)
                    } else {
                        Self::Search(curr.to_string())
                    }
                }
                _ => Self::Search(curr.to_string()),
            },
        }
    }
}

pub fn solve_part_1() -> i32 {
    fs::read_to_string("src/year_2024/inputs/day_3.txt")
        .expect("Oops, could not open file.")
        .chars()
        .fold((State::Search("".to_string()), 0), |mut acc, curr| {
            acc.0 = acc.0.change_state(curr);
            if let State::Num(n1, n2, finished) = &acc.0 {
                match (n1, n2, finished) {
                    (Some(f), Some(s), true) => {
                        acc.1 += f.parse::<i32>().unwrap() * s.parse::<i32>().unwrap();
                        acc.0 = State::Search(curr.to_string());
                    }
                    _ => {}
                }
            }

            acc
        })
        .1
}


#[derive(Debug)]
enum StatePartTwo {
    Search(String, bool),
    Num(Option<String>, Option<String>, bool),
}

impl StatePartTwo {
    fn change_state(&self, curr: char) -> StatePartTwo {
        match self {
            Self::Search(acc, go_to_num) => {
                let new = format!("{}{}", acc, curr);

                if *go_to_num && new.ends_with("mul(") {
                    Self::Num(Some("".to_string()), None, false)
                } else if new.ends_with("do()") {
                    Self::Search(new, true)
                } else if new.ends_with("don't()") {
                    Self::Search(new, false)
                } else {
                    Self::Search(new, *go_to_num)
                }
            }
            Self::Num(num1, num2, _) => match (num1, num2) {
                (Some(v), None) => {
                    if curr.is_numeric() {
                        Self::Num(Some(format!("{}{}", v, curr)), None, false)
                    } else if curr == ',' {
                        Self::Num(Some(v.clone()), Some("".to_string()), false)
                    } else {
                        Self::Search(curr.to_string(), true)
                    }
                }
                (v, Some(n)) => {
                    if curr.is_numeric() {
                        Self::Num(v.clone(), Some(format!("{}{}", n, curr)), false)
                    } else if curr == ')' {
                        Self::Num(v.clone(), Some(n.clone()), true)
                    } else {
                        Self::Search(curr.to_string(), true)
                    }
                }
                _ => Self::Search(curr.to_string(), true),
            },
        }
    }
}

pub fn solve_part_2() -> i32 {
    fs::read_to_string("src/year_2024/inputs/day_3.txt")
        .expect("Oops, could not open file.")
        .chars()
        .fold((StatePartTwo::Search("".to_string(), true), 0), |mut acc, curr| {
            acc.0 = acc.0.change_state(curr);

            if let StatePartTwo::Num(n1, n2, finished) = &acc.0 {
                match (n1, n2, finished) {
                    (Some(f), Some(s), true) => {
                        acc.1 += f.parse::<i32>().unwrap() * s.parse::<i32>().unwrap();
                        acc.0 = StatePartTwo::Search(curr.to_string(), true);
                    }
                    _ => {}
                }
            }

            acc
        })
        .1
}

