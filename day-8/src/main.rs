use std::fs;

use itertools::Itertools;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split("\n")
        .map(|s| s.split(" | ").collect::<Vec<&str>>())
        .collect::<Vec<_>>();

    // 1 = 2 segments
    // 4 = 4 segments
    // 7 = 3 segments
    // 8 = 7 segments

    let mut count = 0;

    for line in c {
        let out_values = line[1]
            .split(" ")
            .map(|s| s.chars().count())
            .collect::<Vec<_>>();

        // println!("{:?}", out_values);
        for v in out_values {
            if v == 2 || v == 4 || v == 3 || v == 7 {
                count += 1;
            }
        }
    }

    println!("result={}", count);
}

#[derive(Debug, Clone)]
struct Translation {
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char,
}

impl Translation {
    pub fn translate(&self, c: char) -> char {
        match c {
            'a' => self.a,
            'b' => self.b,
            'c' => self.c,
            'd' => self.d,
            'e' => self.e,
            'f' => self.f,
            'g' => self.g,
            _ => panic!("unknown char"),
        }
    }
}

struct Number {
    number: i32,
    segments: Vec<char>,
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split("\n")
        .map(|s| s.split(" | ").collect::<Vec<&str>>())
        .collect::<Vec<_>>();

    // 1 = 2 segments
    // 4 = 4 segments
    // 7 = 3 segments
    // 8 = 7 segments

    let seg = vec!["a", "b", "c", "d", "e", "f", "g"];

    let mut final_value = 0;

    for line in c {
        let mut found_translation: Option<Translation> = None;
        for a in seg.clone().into_iter().permutations(7) {
            if a.len() == 7 {
                let try_vals = line[0].split(" ").collect::<Vec<_>>();

                // println!("trying {:?}", a);
                let t = Translation {
                    a: a[0].chars().next().unwrap(),
                    b: a[1].chars().next().unwrap(),
                    c: a[2].chars().next().unwrap(),
                    d: a[3].chars().next().unwrap(),
                    e: a[4].chars().next().unwrap(),
                    f: a[5].chars().next().unwrap(),
                    g: a[6].chars().next().unwrap(),
                };
                // println!("translation: {:#?}", t.clone());

                let mut got_numbers = vec![];
                let mut good = true;

                for v in try_vals {
                    let nr = valid_number(t.clone(), v);

                    if !nr.valid {
                        good = false;
                        break;
                    }
                    if got_numbers.contains(&nr.number.unwrap()) {
                        good = false;
                        break;
                    }

                    got_numbers.push(nr.number.unwrap());
                }

                if good {
                    // println!("{:?}", got_numbers);
                    found_translation = Some(t);
                    break;
                }
            }
        }

        // println!("found translation = {:?}", found_translation);

        let out_vals = line[1].split(" ").collect::<Vec<_>>();
        let mut fin_str: String = "".into();
        for num in out_vals {
            let n_transla = valid_number(found_translation.clone().unwrap(), num);

            if !n_transla.valid {
                panic!("translation not valid");
            }
            fin_str = format!("{}{}", fin_str, &n_transla.number.unwrap().to_string());
        }
        let fin_int: i32 = fin_str.parse().unwrap();
        println!("got num {:?}", fin_int);
        final_value += fin_int;
    }

    println!("result={}", final_value);
}

fn do_number() -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    numbers.push(Number {
        number: 0,
        segments: vec!['a', 'b', 'c', 'e', 'f', 'g'],
    });
    numbers.push(Number {
        number: 1,
        segments: vec!['c', 'f'],
    });
    numbers.push(Number {
        number: 2,
        segments: vec!['a', 'c', 'd', 'e', 'g'],
    });
    numbers.push(Number {
        number: 3,
        segments: vec!['a', 'c', 'd', 'f', 'g'],
    });
    numbers.push(Number {
        number: 4,
        segments: vec!['b', 'c', 'd', 'f'],
    });
    numbers.push(Number {
        number: 5,
        segments: vec!['a', 'b', 'd', 'f', 'g'],
    });
    numbers.push(Number {
        number: 6,
        segments: vec!['a', 'b', 'd', 'e', 'f', 'g'],
    });
    numbers.push(Number {
        number: 7,
        segments: vec!['a', 'c', 'f'],
    });
    numbers.push(Number {
        number: 8,
        segments: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
    });
    numbers.push(Number {
        number: 9,
        segments: vec!['a', 'b', 'c', 'd', 'f', 'g'],
    });

    numbers
}

fn valid_number(display_map: Translation, signal: &str) -> NumberResponse {
    let numbers = do_number();

    for n in numbers {
        let mut is_number = true;

        if n.segments.len() != signal.chars().count() {
            continue;
        }

        for s in n.segments {
            let mut is_char = false;
            for sc in signal.chars() {
                let tsc = display_map.translate(sc);
                if tsc == s {
                    is_char = true;
                }
            }

            if !is_char {
                is_number = false;
                continue;
            }
        }

        if is_number {
            // println!("number!! {} {}", n.number, signal);
            return NumberResponse {
                valid: true,
                number: Some(n.number),
            };
        }
    }

    NumberResponse {
        valid: false,
        number: None,
    }
}

struct NumberResponse {
    valid: bool,
    number: Option<i32>,
}
