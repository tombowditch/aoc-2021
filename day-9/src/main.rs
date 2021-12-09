use std::{fs, sync::Arc};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split("\n")
        .map(|s| {
            s.chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let mut risk_level_total = 0;

    for (line_idx, line) in c.iter().enumerate() {
        for (char_idx, cha) in line.iter().enumerate() {
            let mut up: Option<i32> = None;
            let mut down: Option<i32> = None;
            let mut left: Option<i32> = None;
            let mut right: Option<i32> = None;

            // UP
            if line_idx > 0 {
                up = Some(c[line_idx - 1][char_idx]);
            }

            // DOWN
            if line_idx < c.len() - 1 {
                down = Some(c[line_idx + 1][char_idx]);
            }

            // LEFT
            if char_idx > 0 {
                left = Some(line[char_idx - 1]);
            }

            // RIGHT
            if char_idx < line.len() - 1 {
                right = Some(line[char_idx + 1]);
            }

            println!(
                "char {}, up {:?} down {:?} left {:?} right {:?}",
                cha, up, down, left, right
            );

            if cha < &up.unwrap_or(999)
                && cha < &down.unwrap_or(999)
                && cha < &left.unwrap_or(999)
                && cha < &right.unwrap_or(999)
            {
                println!("{}", cha);

                risk_level_total += cha + 1;
            }
        }
    }

    println!("risk level total: {}", risk_level_total);
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split("\n")
        .map(|s| {
            s.chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let mut basins = vec![];

    for (line_idx, line) in c.iter().enumerate() {
        for (char_idx, cha) in line.iter().enumerate() {
            let mut up: Option<i32> = None;
            let mut down: Option<i32> = None;
            let mut left: Option<i32> = None;
            let mut right: Option<i32> = None;

            // UP
            if line_idx > 0 {
                up = Some(c[line_idx - 1][char_idx]);
            }

            // DOWN
            if line_idx < c.len() - 1 {
                down = Some(c[line_idx + 1][char_idx]);
            }

            // LEFT
            if char_idx > 0 {
                left = Some(line[char_idx - 1]);
            }

            // RIGHT
            if char_idx < line.len() - 1 {
                right = Some(line[char_idx + 1]);
            }

            // println!(
            //     "char {}, up {:?} down {:?} left {:?} right {:?}",
            //     cha, up, down, left, right
            // );

            if cha < &up.unwrap_or(999)
                && cha < &down.unwrap_or(999)
                && cha < &left.unwrap_or(999)
                && cha < &right.unwrap_or(999)
            {
                // start of basin
                let mut covered: Vec<(usize, usize)> = vec![];

                in_basin_check(char_idx, line_idx, c.clone(), &mut covered);

                println!("basin count: {}", covered.len());
                println!("covered {:?}", covered);

                basins.push(covered.len());
            }
        }
    }

    basins.sort();
    basins.reverse();
    println!("basins: {:?}", basins);

    println!("result={}", basins[0] * basins[1] * basins[2])
}

fn in_basin_check(
    char_idx: usize,
    line_idx: usize,
    field: Vec<Vec<i32>>,
    covered: &mut Vec<(usize, usize)>,
) {
    let mut up = 9;
    let mut down = 9;
    let mut left = 9;
    let mut right = 9;

    if covered.contains(&(char_idx, line_idx)) {
        return;
    }

    // UP
    if line_idx > 0 {
        up = field[line_idx - 1][char_idx];
    }

    // DOWN
    if line_idx < field.len() - 1 {
        down = field[line_idx + 1][char_idx];
    }

    // LEFT
    if char_idx > 0 {
        left = field[line_idx][char_idx - 1];
    }

    // RIGHT
    if char_idx < field[line_idx].len() - 1 {
        right = field[line_idx][char_idx + 1];
    }

    (*covered).push((char_idx, line_idx));

    if up < 9 && up > field[line_idx][char_idx] {
        in_basin_check(char_idx, line_idx - 1, field.clone(), covered);
    }

    if down < 9 && down > field[line_idx][char_idx] {
        in_basin_check(char_idx, line_idx + 1, field.clone(), covered);
    }

    if left < 9 && left > field[line_idx][char_idx] {
        in_basin_check(char_idx - 1, line_idx, field.clone(), covered);
    }

    if right < 9 && right > field[line_idx][char_idx] {
        in_basin_check(char_idx + 1, line_idx, field.clone(), covered);
    }
}
