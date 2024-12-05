use std::fs;

const LINE_LEN: usize = 141;

pub fn solve_part_1() -> i32 {
    let map = fs::read_to_string("src/year_2024/inputs/day_4.txt")
        .expect("Oops, could not open file.")
        .chars()
        .collect::<Vec<char>>();

    map.iter()
        .enumerate()
        .filter_map(|(i, &x)| {
            if x != 'X' {
                return None;
            }

            let right = if i + 3 < map.len() {
                (map[(i + 1)..=(i + 3)].iter().collect::<String>() == "MAS".to_string()) as i32
            } else {
                0
            };

            let left = if i >= 3 {
                (map[(i - 3)..i].iter().collect::<String>() == "SAM".to_string()) as i32
            } else {
                0
            };

            let up = if i >= LINE_LEN * 3 {
                (map[i - LINE_LEN] == 'M'
                    && map[i - LINE_LEN * 2] == 'A'
                    && map[i - LINE_LEN * 3] == 'S') as i32
            } else {
                0
            };

            let down = if i + LINE_LEN * 3 < map.len() {
                (map[i + LINE_LEN] == 'M'
                    && map[i + LINE_LEN * 2] == 'A'
                    && map[i + LINE_LEN * 3] == 'S') as i32
            } else {
                0
            };

            let left_up = search_diagonally(&map, i, false, false) as i32;
            let right_up = search_diagonally(&map, i, false, true) as i32;

            let left_down = search_diagonally(&map, i, true, false) as i32;
            let right_down = search_diagonally(&map, i, true, true) as i32;

            Some(right + left + left_down + right_down + up + down + left_up + right_up)
        })
        .sum()
}

fn search_diagonally(map: &Vec<char>, begin: usize, down: bool, right: bool) -> bool {
    let calc_dir = |a: usize, b: usize, inc: bool| {
        if inc && (a + b) < map.len() {
            return (a + b, true);
        }

        if !inc && a >= b {
            return (a - b, true);
        };

        (0, false)
    };

    for target in ['M', 'A', 'S'] {
        let i: usize = match target {
            'M' => 1,
            'A' => 2,
            'S' => 3,
            _ => 0,
        };

        let (dir1, cont1) = calc_dir(begin, i, right);
        let (dir, cont) = calc_dir(dir1, LINE_LEN * i, down);

        if !(cont && cont1) || map[dir] != target {
            return false;
        }
    }

    true
}

pub fn solve_part_2() -> i32 {
    let map = fs::read_to_string("src/year_2024/inputs/day_4.txt")
        .expect("Oops, could not open file.")
        .chars()
        .collect::<Vec<char>>();

    map.iter()
        .enumerate()
        .filter_map(|(i, &x)| {
            if x != 'A' || i < LINE_LEN + 1 || i + LINE_LEN + 1 >= map.len() {
                return None;
            }

            let mas_one = String::from(format!(
                "{}A{}",
                map[i - LINE_LEN - 1],
                map[i + LINE_LEN + 1],
            ));

            let mas_two = String::from(format!(
                "{}A{}",
                map[i - LINE_LEN + 1],
                map[i + LINE_LEN - 1],
            ));

            let mas = "MAS".to_string();
            let sam = "SAM".to_string();

            Some(((mas_one == mas || mas_one == sam) && (mas_two == mas || mas_two == sam)) as i32)
        })
        .sum()
}
