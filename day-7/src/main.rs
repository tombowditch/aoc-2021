use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("before sort {:?}", c);
    // sort in order
    c.sort();
    println!("after sort {:?}", c);

    let median = c[c.len() / 2];

    println!("median= {}", median);

    let mut used_fuel = 0;
    for crab_loc in c {
        let mut f = median - crab_loc;
        if f < 0 {
            f *= -1;
        }
        used_fuel += f;
    }

    println!("used fuel {}", used_fuel);
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("before sort {:?}", c);
    // sort in order
    c.sort();
    println!("after sort {:?}", c);

    let start = c[0];
    let end = c[c.len() - 1];

    println!("start={}, end={}", start, end);

    let mut tot_fuel = 0;

    for pos in start..=end {
        println!("checking pos={}", pos);
        let mut used_fuel = 0;
        for crab_loc in c.clone() {
            let mut f = pos - crab_loc;
            if f < 0 {
                f *= -1;
            }
            used_fuel += triangular_number(f.try_into().unwrap());
        }

        if used_fuel < tot_fuel {
            tot_fuel = used_fuel;
        }

        // start
        if tot_fuel == 0 {
            tot_fuel = used_fuel;
        }
    }

    println!("used fuel {}", tot_fuel);
}

fn triangular_number(n: f64) -> i32 {
    ((n * (n + 1 as f64)) / 2 as f64) as i32
}
