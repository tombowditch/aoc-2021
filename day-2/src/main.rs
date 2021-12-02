use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str
        .split("\n")
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut x = 0;
    let mut depth = 0;

    for line in c {
        let (direction, steps) = (line[0], line[1].parse::<i32>().unwrap());

        println!("going {} steps {}", steps, direction);

        match direction {
            "forward" => x += steps,
            "up" => depth -= steps,
            "down" => depth += steps,
            _ => panic!("unknown direction"),
        }
    }

    println!("final destination is x={}, depth={}", x, depth);
    println!("final = {}", x * depth)
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str
        .split("\n")
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in c {
        let (direction, steps) = (line[0], line[1].parse::<i32>().unwrap());

        println!("going {} steps {}", steps, direction);

        match direction {
            "forward" => {
                x += steps;
                depth += aim * steps;
            }
            "up" => aim -= steps,
            "down" => aim += steps,
            _ => panic!("unknown direction"),
        }
    }

    println!("final destination is x={}, depth={}", x, depth);
    println!("final = {}", x * depth)
}
