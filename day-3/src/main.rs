use std::{collections::HashMap, fs};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str
        .split("\n")
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let bit_count = c[0].len();

    println!("{:#?}", c[0]);
    println!("bit count = {}", bit_count);

    let mut gamma_rate = vec![];
    let mut epsilon_rate = vec![];

    for i in 0..bit_count {
        let mut cache: HashMap<String, i32> = HashMap::new();
        println!("bitcount {}", i);
        for line in c.clone() {
            let bit = line[i];
            cache
                .entry(bit.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let most_common_bit = cache
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _)| k)
            .unwrap();

        let least_common_bit = cache
            .iter()
            .min_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _)| k)
            .unwrap();

        println!("{:#?}", cache);
        println!(
            "most common is {}, least common is {}",
            most_common_bit, least_common_bit
        );

        gamma_rate.push(most_common_bit.clone());
        epsilon_rate.push(least_common_bit.clone());
    }

    println!("gamma {:#?}", gamma_rate.join(""));
    println!("eps {:#?}", epsilon_rate.join(""));

    let dec_gamma_rate = isize::from_str_radix(&gamma_rate.join(""), 2).unwrap();
    let dec_epsilon_rate = isize::from_str_radix(&epsilon_rate.join(""), 2).unwrap();

    println!("gamma dec {:#?}", dec_gamma_rate);
    println!("eps dec {:#?}", dec_epsilon_rate);

    println!("result: {}", dec_gamma_rate * dec_epsilon_rate);
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split("\n")
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let bit_count = c[0].len();

    println!("{:#?}", c[0]);
    println!("bit count = {}", bit_count);

    let oxygen_gen_rating = calc_value(bit_count, c.clone(), Method::MostCommon);
    let c02_scrubber_rating = calc_value(bit_count, c, Method::LeastCommon);

    println!("oxygen gen rating: {}", oxygen_gen_rating);
    println!("c02 scrubber rating: {}", c02_scrubber_rating);

    println!("result: {}", oxygen_gen_rating * c02_scrubber_rating);
}

fn calc_value(bit_count: usize, mut c: Vec<Vec<char>>, method: Method) -> isize {
    for i in 0..bit_count {
        let mut cache: HashMap<String, i32> = HashMap::new();
        println!("bitcount {}", i);

        println!("c = {:#?}", c);

        for line in c.clone() {
            let bit = line[i];
            cache
                .entry(bit.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let filter_type = filter_bits(cache, &method);

        println!("most common is {}", filter_type);

        let mut new_list: Vec<Vec<char>> = vec![];
        for line in c.clone() {
            if line[i] == filter_type.chars().collect::<Vec<char>>()[0] {
                new_list.push(line);
            }
        }

        c = new_list;
    }

    if c.len() != 1 {
        panic!("too many/not enough lines");
    }

    println!("final c = {:#?}", c[0]);

    let final_str = c[0].clone().into_iter().collect::<String>();

    let dec = isize::from_str_radix(&final_str, 2).unwrap();
    println!("result: {}", dec);

    return dec;
}

enum Method {
    MostCommon,
    LeastCommon,
}

fn filter_bits(cache: HashMap<String, i32>, method: &Method) -> String {
    match method {
        Method::MostCommon => {
            if cache.get("0").unwrap_or(&0) == cache.get("1").unwrap_or(&0) {
                // what a hack...
                return "1".to_string();
            }

            cache
                .iter()
                .max_by(|a, b| a.1.cmp(b.1))
                .map(|(k, _)| k)
                .unwrap()
                .clone()
        }
        Method::LeastCommon => {
            if cache.get("0").unwrap_or(&0) == cache.get("1").unwrap_or(&0) {
                // what a hack...
                return "0".to_string();
            }

            cache
                .iter()
                .min_by(|a, b| a.1.cmp(b.1))
                .map(|(k, _)| k)
                .unwrap()
                .clone()
        }
    }
}
