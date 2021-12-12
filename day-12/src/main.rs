use std::{collections::HashMap, fs, ops::Index, path};

fn main() {
    part_one();
    part_two();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Cave {
    Big(String),
    Small(String),
    Start,
    End,
}

impl Cave {
    fn convert(s: &str) -> Cave {
        if s == "start" {
            Cave::Start
        } else if s == "end" {
            Cave::End
        } else {
            let cha = s.chars().next().unwrap();

            if cha.is_uppercase() {
                Cave::Big(s.to_string())
            } else {
                Cave::Small(s.to_string())
            }
        }
    }

    fn is_small(&self) -> bool {
        match self {
            Cave::Small(_) => true,
            _ => false,
        }
    }

    fn is_big(&self) -> bool {
        match self {
            Cave::Big(_) => true,
            _ => false,
        }
    }
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str.split("\n").collect::<Vec<_>>();

    let mut possible_caves: HashMap<Cave, Vec<Cave>> = HashMap::new();

    for line in c {
        let spl = line.split("-").collect::<Vec<_>>();

        let mut input = spl[0];
        let mut output = spl[1];

        let input_cave = Cave::convert(input);
        let output_cave = Cave::convert(output);

        possible_caves
            .entry(input_cave.clone())
            .or_insert(vec![])
            .push(output_cave.clone());

        possible_caves
            .entry(output_cave)
            .or_insert(vec![])
            .push(input_cave);
    }

    println!("{:?}", possible_caves);

    let mut path = vec![Cave::Start];

    let mut all_paths: Vec<Vec<Cave>> = vec![];

    pick_next(
        possible_caves.clone(),
        possible_caves.get(&Cave::Start).unwrap().to_vec(),
        &mut path,
        &mut all_paths,
    );

    println!("we have {} possible paths", all_paths.len());
}

fn pick_next(
    paths: HashMap<Cave, Vec<Cave>>,
    caves: Vec<Cave>,
    path: &mut Vec<Cave>,
    all_paths: &mut Vec<Vec<Cave>>,
) {
    for next_cave in caves.clone() {
        // visited small cave >1 ?
        if path.contains(&next_cave) && next_cave.is_small() {
            continue;
        }

        if next_cave == Cave::Start {
            continue;
        }

        let mut new_path = path.clone();
        new_path.push(next_cave.clone());

        if next_cave == Cave::End {
            println!("path i took: {:?}", new_path);
            all_paths.push(new_path);
            continue;
        }

        pick_next(
            paths.clone(),
            paths.get(&next_cave).unwrap().to_vec(),
            &mut new_path,
            all_paths,
        );
    }
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str.split("\n").collect::<Vec<_>>();

    let mut possible_caves: HashMap<Cave, Vec<Cave>> = HashMap::new();

    for line in c {
        let spl = line.split("-").collect::<Vec<_>>();

        let mut input = spl[0];
        let mut output = spl[1];

        let input_cave = Cave::convert(input);
        let output_cave = Cave::convert(output);

        possible_caves
            .entry(input_cave.clone())
            .or_insert(vec![])
            .push(output_cave.clone());

        possible_caves
            .entry(output_cave)
            .or_insert(vec![])
            .push(input_cave);
    }

    println!("{:?}", possible_caves);

    let mut path = vec![Cave::Start];

    let mut all_paths: Vec<Vec<Cave>> = vec![];

    let mut small_caves = vec![];

    for (k, _) in possible_caves.clone() {
        if k.is_small() {
            small_caves.push(k);
        }
    }

    for dbl_small in small_caves {
        pick_next_p2(
            possible_caves.clone(),
            possible_caves.get(&Cave::Start).unwrap().to_vec(),
            &mut path,
            &mut all_paths,
            dbl_small,
        );
    }

    println!("we have {} possible paths", all_paths.len());
}

fn pick_next_p2(
    paths: HashMap<Cave, Vec<Cave>>,
    caves: Vec<Cave>,
    path: &mut Vec<Cave>,
    all_paths: &mut Vec<Vec<Cave>>,
    dbl_small: Cave,
) {
    for next_cave in caves.clone() {
        // visited small cave >1  and not double small ?
        if path.contains(&next_cave) && next_cave.is_small() && next_cave != dbl_small {
            continue;
        }

        let mut d_count = 0;
        for p in path.clone() {
            if p.clone() == dbl_small {
                d_count += 1;
            }
        }

        if d_count > 2 {
            continue;
        }

        if next_cave == Cave::Start {
            continue;
        }

        let mut new_path = path.clone();
        new_path.push(next_cave.clone());

        if next_cave == Cave::End {
            println!("path i took: {:?}", new_path);
            if !all_paths.contains(&new_path) {
                all_paths.push(new_path);
            }
            continue;
        }

        pick_next_p2(
            paths.clone(),
            paths.get(&next_cave).unwrap().to_vec(),
            &mut new_path,
            all_paths,
            dbl_small.clone(),
        );
    }
}
