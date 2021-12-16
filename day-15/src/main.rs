use pathfinding::prelude::dijkstra;
use std::fs;
use std::time::Instant;

fn main() {
    part_one();
    let begin = Instant::now();
    part_two();
    println!("Execution time: {:?}", Instant::now().duration_since(begin));
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|c| {
            c.chars()
                .map(|cc| cc.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let max_coord = (c.len() - 1, c[0].len() - 1);

    let res = dijkstra(
        &(0, 0),
        |current_pos| {
            let mut successors = vec![];

            // LEFT
            if current_pos.0 > 0 {
                let left_pos = (current_pos.0 - 1, current_pos.1);
                successors.push((left_pos, c[left_pos.0][left_pos.1]));
            }

            // RIGHT
            if current_pos.0 < max_coord.0 {
                let right_pos = (current_pos.0 + 1, current_pos.1);
                successors.push((right_pos, c[right_pos.0][right_pos.1]));
            }

            // UP
            if current_pos.1 > 0 {
                let up_pos = (current_pos.0, current_pos.1 - 1);
                successors.push((up_pos, c[up_pos.0][up_pos.1]));
            }

            // DOWN
            if current_pos.1 < max_coord.1 {
                let down_pos = (current_pos.0, current_pos.1 + 1);
                successors.push((down_pos, c[down_pos.0][down_pos.1]));
            }

            successors
        },
        |p| *p == max_coord,
    );

    println!("{:?}", res.unwrap().1);
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|c| {
            c.chars()
                .map(|cc| cc.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut big_cavern = vec![vec![0usize; c[0].len() * 5]; c.len() * 5];

    for i in 0..c.len() {
        for j in 0..c[0].len() {
            big_cavern[i][j] = c[i][j];
        }
    }

    let original_max = (c.len() - 1, c[0].len() - 1);

    for i in 1..=4 {
        for line_idx in 0..=original_max.0 {
            for char_idx in 0..=original_max.1 {
                // X         X
                // 0 1 2 3 4 5 6 7 8
                // O O O O O N N N N
                big_cavern[line_idx][char_idx + (i * (original_max.1 + 1))] = {
                    let mut val = c[line_idx][char_idx] + i;

                    if val > 9 {
                        val = val - 9;
                    }

                    val
                };
            }
        }
    }

    for i in 1..=4 {
        for line_idx in 0..=original_max.0 {
            for char_idx in 0..big_cavern[0].len() {
                big_cavern[line_idx + (i * (original_max.0 + 1))][char_idx] = {
                    let mut val = big_cavern[line_idx][char_idx] + i;

                    if val > 9 {
                        val = val - 9;
                    }

                    val
                };
            }
        }
    }

    // println!("{:?}", big_cavern);

    let max_coord = (big_cavern.len() - 1, big_cavern[0].len() - 1);

    let res = dijkstra(
        &(0, 0),
        |current_pos| {
            let mut successors = vec![];

            // LEFT
            if current_pos.0 > 0 {
                let left_pos = (current_pos.0 - 1, current_pos.1);
                successors.push((left_pos, big_cavern[left_pos.0][left_pos.1]));
            }

            // RIGHT
            if current_pos.0 < max_coord.0 {
                let right_pos = (current_pos.0 + 1, current_pos.1);
                successors.push((right_pos, big_cavern[right_pos.0][right_pos.1]));
            }

            // UP
            if current_pos.1 > 0 {
                let up_pos = (current_pos.0, current_pos.1 - 1);
                successors.push((up_pos, big_cavern[up_pos.0][up_pos.1]));
            }

            // DOWN
            if current_pos.1 < max_coord.1 {
                let down_pos = (current_pos.0, current_pos.1 + 1);
                successors.push((down_pos, big_cavern[down_pos.0][down_pos.1]));
            }

            successors
        },
        |p| *p == max_coord,
    );

    println!("{:?}", res.unwrap().1);
}
