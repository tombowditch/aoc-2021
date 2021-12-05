use std::fs;

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str
        .split("\n")
        .map(|s| s.split(" -> ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    println!("{:#?}", c);

    let mut grid = vec![vec![0usize; 1000]; 1000];

    for journey in c {
        let start_coords = journey[0]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let end_coords = journey[1]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let start_x = start_coords[0];
        let start_y = start_coords[1];

        let end_x = end_coords[0];
        let end_y = end_coords[1];

        // println!(
        //     "starting at x={} y={}, going to x={} y={}",
        //     start_x, start_y, end_x, end_y
        // );

        if start_x != end_x && start_y != end_y {
            println!("not a straight line");
            continue;
        }

        // println!("drawing");

        // is it moving vertically or horizontally

        if start_y == end_y {
            // horizontal e.g. 0,9 to 5,9
            // println!("horizontal");

            grid[start_y] = grid[start_y]
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    if (i >= start_x && i <= end_x) || i >= end_x && i <= start_x {
                        *v + 1
                    } else {
                        *v
                    }
                })
                .collect();
        } else {
            // vertical
            // println!("vertical");

            grid = grid
                .iter()
                .enumerate()
                .map(|(i, line)| {
                    if (i >= start_y && i <= end_y) || (i >= end_y && i <= start_y) {
                        line.iter()
                            .enumerate()
                            .map(|(j, v)| {
                                if (j >= start_x && j <= end_x) || (j >= end_x && j <= start_x) {
                                    *v + 1
                                } else {
                                    *v
                                }
                            })
                            .collect()
                    } else {
                        line.clone()
                    }
                })
                .collect();
        }
    }

    println!("final grid:");
    for line in grid.clone() {
        // println!("{:?}", line);
    }

    let mut count = 0;
    for line in grid {
        for v in line {
            if v >= 2 {
                count += 1;
            }
        }
    }

    println!("result = {}", count);
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let c = input_str
        .split("\n")
        .map(|s| s.split(" -> ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    println!("{:#?}", c);

    let mut grid = vec![vec![0usize; 1000]; 1000];

    for journey in c {
        let start_coords = journey[0]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let end_coords = journey[1]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let start_x = start_coords[0];
        let start_y = start_coords[1];

        let end_x = end_coords[0];
        let end_y = end_coords[1];

        println!(
            "starting at x={} y={}, going to x={} y={}",
            start_x, start_y, end_x, end_y
        );

        println!("drawing");

        // is it moving vertically or horizontally or diagonally

        if start_y == end_y {
            // horizontal e.g. 0,9 to 5,9
            println!("horizontal");

            grid[start_y] = grid[start_y]
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    if (i >= start_x && i <= end_x) || i >= end_x && i <= start_x {
                        *v + 1
                    } else {
                        *v
                    }
                })
                .collect();
        } else if start_x == end_x {
            // vertical
            println!("vertical");

            grid = grid
                .iter()
                .enumerate()
                .map(|(i, line)| {
                    if (i >= start_y && i <= end_y) || (i >= end_y && i <= start_y) {
                        line.iter()
                            .enumerate()
                            .map(|(j, v)| {
                                if (j >= start_x && j <= end_x) || (j >= end_x && j <= start_x) {
                                    *v + 1
                                } else {
                                    *v
                                }
                            })
                            .collect()
                    } else {
                        line.clone()
                    }
                })
                .collect();
        } else {
            // diagonal at only 45 degrees
            println!("diagonal");

            let points = gen_diagonal_line(start_x, end_x, start_y, end_y);

            for p in points {
                grid[p.1][p.0] += 1;
            }
        }
    }

    println!("final grid:");
    for line in grid.clone() {
        println!("{:?}", line);
    }

    let mut count = 0;
    for line in grid {
        for v in line {
            if v >= 2 {
                count += 1;
            }
        }
    }

    println!("result = {}", count);
}

fn main() {
    part_one();
    part_two();

    // println!("{:#?}", gen_diagonal_line(1, 3, 1, 3));
    // println!("{:#?}", gen_diagonal_line(9, 7, 9, 7));

    // println!("{:#?}", gen_diagonal_line(1, 1, 1, 3));
    // println!("{:#?}", gen_diagonal_line(1, 1, 3, 1));
    // println!("{:#?}", gen_diagonal_line(9, 7, 7, 7));
    // println!("{:#?}", gen_diagonal_line(1, 3, 1, 1));
    // println!("{:#?}", gen_diagonal_line(8, 0, 0, 8));
}

/*
 ! super verbose on purpose
*/
fn gen_diagonal_line(
    start_x: usize,
    end_x: usize,
    start_y: usize,
    end_y: usize,
) -> Vec<(usize, usize)> {
    let mut line = vec![];

    // example 0,+1
    //               x,y    x,y
    // An entry like 1,1 -> 1,3
    // step 1: 1,1 -> 1,2: 0,+1
    // step 2: 1,2 -> 1,3: 0,+1

    if start_x == end_x && end_y > start_y {
        // going down
        for y in start_y..=end_y {
            line.push((start_x, y));
        }
        return line;
    }

    // example 0,-1
    //               x,y    x,y
    // An entry like 1,3 -> 1,1
    // step 1: 1,3 -> 1,2: 0,-1
    // step 2: 1,2 -> 1,1: 0,-1

    if start_x == end_x && end_y < start_y {
        // going up
        for y in (end_y..=start_y).rev() {
            line.push((start_x, y));
        }
        return line;
    }

    // example -1,0
    //               x,y    x,y
    //An entry like 9,7 -> 7,7
    // step 1: 9,7 -> 8,7: -1,0
    // step 2: 8,7 -> 7,7: -1,0

    if start_y == end_y && end_x < start_x {
        // going left
        for x in (end_x..=start_x).rev() {
            line.push((x, start_y));
        }
        return line;
    }

    // example +1,0
    //               x,y    x,y
    // An entry like 1,1 -> 3,1
    // step 1: 1,1 -> 2,1: +1,0
    // step 2: 2,1 -> 3,1: +1,0

    if start_y == end_y && end_x > start_x {
        // going right
        for x in start_x..=end_x {
            line.push((x, start_y));
        }
        return line;
    }

    // example -1,+1
    //               x,y    x,y
    // An entry like 8,0 -> 0,8
    // step 1: 8,0 -> 7,1: -1,+1
    // step 2: 7,1 -> 6,2: -1,+1

    if start_x > end_x && start_y < end_y {
        // going down left
        let mut x = start_x;
        let mut y = start_y;

        while x > end_x {
            line.push((x, y));
            x -= 1;
            y += 1;
        }
        line.push((end_x, end_y));
        return line;
    }

    // example -1,-1
    //               x,y    x,y
    // An entry like 8,8 -> 0,0
    // step 1: 8,8 -> 7,7: -1,-1
    // step 2: 7,7 -> 6,6: -1,-1

    if start_x > end_x && start_y > end_y {
        // going up left
        let mut x = start_x;
        let mut y = start_y;

        while x > end_x {
            line.push((x, y));
            x -= 1;
            y -= 1;
        }
        line.push((end_x, end_y));
        return line;
    }

    // example +1,-1
    //               x,y    x,y
    // An entry like 0,8 -> 8,0
    // step 1: 0,8 -> 1,7: +1,-1
    // step 2: 1,7 -> 2,6: +1,-1

    if start_x < end_x && start_y > end_y {
        // going up right
        let mut x = start_x;
        let mut y = start_y;

        while x < end_x {
            line.push((x, y));
            x += 1;
            y -= 1;
        }
        line.push((end_x, end_y));
        return line;
    }

    // example +1,+1
    //               x,y    x,y
    // An entry like 0,0 -> 8,8
    // step 1: 0,0 -> 1,1: +1,+1
    // step 2: 1,1 -> 2,2: +1,+1

    if start_x < end_x && start_y < end_y {
        // going down right
        let mut x = start_x;
        let mut y = start_y;

        while x < end_x {
            line.push((x, y));
            x += 1;
            y += 1;
        }
        line.push((end_x, end_y));
        return line;
    }

    unimplemented!();
}
