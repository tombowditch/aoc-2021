use std::time::Instant;
use std::{collections::HashMap, fs};

fn main() {
    part_one();
    let begin = Instant::now();
    part_two();
    println!("Execution time: {:?}", Instant::now().duration_since(begin));
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str.split("\n").collect::<Vec<_>>();

    let mut polymer = c[0].to_string();

    c = c[2..].to_vec();

    println!("polymer: {}", polymer);
    println!("c: {:?}", c);

    let mut translations: HashMap<String, String> = HashMap::new();

    for t in c {
        let t_split = t.split(" -> ").collect::<Vec<_>>();
        translations.insert(t_split[0].to_string(), t_split[1].to_string());
    }

    println!("translations: {:?}", translations);

    for _ in 1..=10 {
        let polymer_chars = polymer.chars().collect::<Vec<_>>();

        let mut new_polymer = vec![];

        for pair in polymer_chars.windows(2) {
            println!("pair: {:?}", pair);
            let combined_pair = pair.iter().collect::<String>();

            if let Some(insert) = translations.get(&combined_pair) {
                let new = format!("{}{}", pair[0], insert);
                new_polymer.push(new);
            } else {
                println!("translation does not exist");
            }
        }

        new_polymer.push(polymer_chars[polymer_chars.len() - 1].to_string());

        let new_p = new_polymer.join("");

        polymer = new_p;
    }

    let mut count: HashMap<char, i32> = HashMap::new();

    for pc in polymer.chars() {
        let count_val = count.entry(pc).or_insert(0);
        *count_val += 1;
    }

    println!("FINAL polymer: {:?}", polymer);
    println!("FINAL polymer: {:?}", polymer.len());

    let most_common_bit = count.iter().max_by_key(|a| a.1).map(|(k, v)| v).unwrap();
    let least_common_bit = count.iter().min_by_key(|a| a.1).map(|(k, v)| v).unwrap();

    println!("count: {:#?}", count);

    println!(
        "most common is {}, least common is {}",
        most_common_bit, least_common_bit
    );

    println!("result = {}", most_common_bit - least_common_bit);
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str.split("\n").collect::<Vec<_>>();

    let mut count: HashMap<char, usize> = HashMap::new();
    let mut polymer: HashMap<String, usize> = HashMap::new();

    let mut last_char = ' ';
    for init_pair in c[0].chars().collect::<Vec<char>>().windows(2) {
        count
            .entry(init_pair[0])
            .and_modify(|e| *e += 1)
            .or_insert(1);

        last_char = init_pair[1];

        let init_pair_str = format!("{}{}", init_pair[0], init_pair[1]);
        *polymer.entry(init_pair_str).or_insert(0) += 1;
    }

    count.entry(last_char).and_modify(|e| *e += 1).or_insert(1);

    c = c[2..].to_vec();

    // println!("polymer: {}", polymer);
    // println!("c: {:?}", c);

    let mut translations: HashMap<String, String> = HashMap::new();

    for t in c {
        let t_split = t.split(" -> ").collect::<Vec<_>>();
        translations.insert(t_split[0].to_string(), t_split[1].to_string());
    }

    // println!("translations: {:?}", translations);

    for step in 1..=40 {
        // println!("step: {}", step);

        let mut new_polymer: HashMap<String, usize> = HashMap::new();
        for (pair, amount) in polymer.clone() {
            let ins = translations.get(&pair).unwrap();

            let pair_chars = pair.chars().collect::<Vec<_>>();

            *new_polymer
                .entry(format!("{}{}", pair_chars[0], ins))
                .or_insert(0) += amount;
            *new_polymer
                .entry(format!("{}{}", ins, pair_chars[1]))
                .or_insert(0) += amount;

            count
                .entry(ins.chars().next().unwrap())
                .and_modify(|e| *e += amount)
                .or_insert(amount);
        }

        polymer = new_polymer;
    }

    // println!("FINAL polymer: {:#?}", polymer);
    // println!("count: {:#?}", count);

    let most_common_bit = count.iter().max_by_key(|a| a.1).map(|(k, v)| v).unwrap();
    let least_common_bit = count.iter().min_by_key(|a| a.1).map(|(k, v)| v).unwrap();

    println!("result = {}", most_common_bit - least_common_bit);
}
