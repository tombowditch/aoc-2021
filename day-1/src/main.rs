use std::fs;

fn main() {
    part_one();

    part_two();
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str.split("\n");

    let mut prev: Option<i32> = None;
    let mut increased = 0;

    for line in c {
        let line_number = line.parse::<i32>().unwrap();

        if prev.is_none() {
            prev = Some(line_number);
            continue;
        }

        if line_number > prev.unwrap() {
            increased += 1;
        }
        prev = Some(line_number);
    }

    println!("[part one] increased {} times", increased);
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str
        .split("\n")
        .map(|i| i.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut can_continue = true;
    let mut start_index = 0;

    let mut prev: Option<i32> = None;
    let mut increased = 0;

    while can_continue {
        print!(".");

        if (start_index + 2) > (c.len() - 1) {
            println!("oob");
            can_continue = false;
            continue;
        }

        let (one, two, three) = (c[start_index], c[start_index + 1], c[start_index + 2]);

        let total = one + two + three;

        if prev.is_none() {
            prev = Some(total);
            continue;
        }

        if total > prev.unwrap() {
            increased += 1;
        }
        prev = Some(total);
        start_index += 1;
    }

    println!("[part two] increased {} times", increased);
}
