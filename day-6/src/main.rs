use std::{fs, collections::HashMap};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split(",")
        .map(|s| s.parse::<i8>().unwrap())
        .collect::<Vec<i8>>();

    for d in 1..=80 {
        let mut new_state: Vec<i8> = vec![];
        println!("day {}", d);

        for mut s in c.clone() {
            s -= 1;

            if s == -1 {
                s = 6;
                new_state.push(8);
            }

            new_state.push(s);
        }

        c = new_state;
    }

    // println!("result = {:?}", c);
    println!("result = {:?}", c.len());
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split(",")
        .map(|s| s.parse::<i8>().unwrap())
        .collect::<Vec<i8>>();

    let mut map: HashMap<i8,i64> = HashMap::new();
    for i in 0..=8 {
        map.insert(i, 0);
    }
    
    for s in c.clone() {
        map.insert(s, map[&s] + 1);
    }

    for d in 1..=256 {
        println!("day {}", d);
        let mut new_map: HashMap<i8,i64> = HashMap::new();
        for i in 0..=8 {
            let a = map.get(&i).unwrap().to_owned();
            // println!("t {}, a {}", i,a);
            if i == 0 {
                new_map.insert(6, a);
                new_map.insert(8, a);
                continue;
            }

            new_map.insert(i - 1, new_map.get(&(i-1)).unwrap_or(&0) + a);
        }

        // println!("tmp map = {:?}", new_map);

        map = new_map;
    }

    println!("map: {:?}", map);

    let mut count = 0;
    for i in 0..=8 {
        count += map.get(&i).unwrap();
    }

    println!("result = {}", count);
}
