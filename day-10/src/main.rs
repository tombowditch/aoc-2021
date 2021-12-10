use std::fs;

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

    // ()
    // []
    // {}
    // <>

    let mut syntax_error_score = 0;
    for line in c {
        println!("ln");
        let mut char_cache = vec![];
        let mut corrupt = false;
        for chr in line.clone() {
            if corrupt {
                continue;
            }

            // could be match
            if chr == '(' {
                char_cache.push(')');
            }

            if chr == '[' {
                char_cache.push(']');
            }

            if chr == '{' {
                char_cache.push('}');
            }

            if chr == '<' {
                char_cache.push('>');
            }

            if chr == ')' {
                if char_cache.pop().unwrap() != ')' {
                    syntax_error_score += 3;
                    corrupt = true;
                }
            }

            if chr == ']' {
                if char_cache.pop().unwrap() != ']' {
                    syntax_error_score += 57;
                    corrupt = true;
                }
            }

            if chr == '}' {
                if char_cache.pop().unwrap() != '}' {
                    syntax_error_score += 1197;
                    corrupt = true;
                }
            }

            if chr == '>' {
                if char_cache.pop().unwrap() != '>' {
                    syntax_error_score += 25137;
                    corrupt = true;
                }
            }
        }

        if corrupt {
            println!("{} is corrupt", line.iter().collect::<String>());
        }
    }

    println!("result={}", syntax_error_score);
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str
        .split("\n")
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<_>>>();

    // ()
    // []
    // {}
    // <>

    let mut scores = vec![];
    for line in c {
        let mut score: i64 = 0;
        println!("ln");
        let mut char_cache = vec![];
        let mut corrupt = false;
        for chr in line.clone() {
            if corrupt {
                continue;
            }

            // could be match
            if chr == '(' {
                char_cache.push(')');
            }

            if chr == '[' {
                char_cache.push(']');
            }

            if chr == '{' {
                char_cache.push('}');
            }

            if chr == '<' {
                char_cache.push('>');
            }

            if chr == ')' {
                if char_cache.pop().unwrap() != ')' {
                    corrupt = true;
                }
            }

            if chr == ']' {
                if char_cache.pop().unwrap() != ']' {
                    corrupt = true;
                }
            }

            if chr == '}' {
                if char_cache.pop().unwrap() != '}' {
                    corrupt = true;
                }
            }

            if chr == '>' {
                if char_cache.pop().unwrap() != '>' {
                    corrupt = true;
                }
            }
        }

        if corrupt {
            // println!("{} is corrupt", line.iter().collect::<String>());
            continue;
        }

        if char_cache.len() == 0 {
            println!("finished line");
            continue;
        }

        if char_cache.len() > 0 {
            println!("incomplete line");
            char_cache.reverse();
            println!("to finish: {}", char_cache.iter().collect::<String>());

            for chr in char_cache {
                score *= 5;

                if chr == ')' {
                    score += 1;
                }

                if chr == ']' {
                    score += 2;
                }

                if chr == '}' {
                    score += 3;
                }

                if chr == '>' {
                    score += 4;
                }
            }

            println!("score={}", score);
            scores.push(score);
        }
    }

    scores.sort();

    println!("result={}", scores[scores.len() / 2]);
}
