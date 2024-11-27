use std::fs;

enum Color {
    Red(i32),
    Blue(i32),
    Green(i32),
}

impl Color {
    fn valid(&self) -> bool {
        match self {
            Self::Red(val) => *val <= 12,
            Self::Blue(val) => *val <= 14,
            Self::Green(val) => *val <= 13,
        }
    }

    fn value(&self) -> &i32 {
        match self {
            Color::Red(val) => val,
            Color::Blue(val) => val,
            Color::Green(val) => val,
        }
    }

    fn set_value(&mut self, value: i32) {
        match self {
            Color::Red(val) => *val = value,
            Color::Blue(val) => *val = value,
            Color::Green(val) => *val = value,
        };
    }

    fn set_max(&mut self, val: i32) {
        if val > *self.value() {
            self.set_value(val);
        }
    }
}

pub fn solve_part_1() -> i32 {
    fs::read_to_string("src/inputs/day_2.txt")
        .expect("Oops, no file found.")
        .lines()
        .filter_map(|l| {
            let mut l_split = l.split(":");

            let game = l_split
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let res = l_split.next().unwrap().split(";").fold(
                (Color::Red(0), Color::Blue(0), Color::Green(0)),
                |mut acc, curr| {
                    for cubes in curr.split(",") {
                        let cube_split = cubes.split(" ").collect::<Vec<&str>>();
                        let amount = cube_split[1].parse::<i32>().unwrap();
                        match cube_split[2] {
                            "red" => acc.0.set_max(amount),
                            "blue" => acc.1.set_max(amount),
                            _ => acc.2.set_max(amount),
                        }
                    }

                    println!("{}, {}, {}", acc.0.value(), acc.1.value(), acc.2.value());
                    acc
                },
            );

            if res.0.valid() && res.1.valid() && res.2.valid() {
                Some(game)
            } else {
                None
            }
        })
        .sum::<i32>()
}

pub fn solve_part_2() -> i32 {
    fs::read_to_string("src/inputs/day_2.txt")
        .expect("Oops, no file found.")
        .lines()
        .map(|l| {
            let res = l.split(":").last().unwrap().split(";").fold(
                (Color::Red(0), Color::Blue(0), Color::Green(0)),
                |mut acc, curr| {
                    for cubes in curr.split(",") {
                        let cube_split = cubes.split(" ").collect::<Vec<&str>>();
                        let amount = cube_split[1].parse::<i32>().unwrap();
                        match cube_split[2] {
                            "red" => acc.0.set_max(amount),
                            "blue" => acc.1.set_max(amount),
                            _ => acc.2.set_max(amount),
                        }
                    }

                    acc
                },
            );

            res.0.value() * res.1.value() * res.2.value()
        })
        .sum::<i32>()
}
