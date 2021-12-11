use std::fs;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split("\n")
        .map(|s| {
            s.chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    render_grid(c.clone());

    let mut flashes = 0;

    for step in 1..=100 {
        let mut flashed = [[false; 10]; 10];
        // println!("STEP {}", step);

        let grid = c.clone();
        for line_idx in 0..grid.len() {
            for char_idx in 0..grid[line_idx].len() {
                c[line_idx][char_idx] += 1;
            }
        }

        for line_idx in 0..grid.len() {
            for char_idx in 0..grid[line_idx].len() {
                // println!("line_idx={}, char_idx={}", line_idx, char_idx);
                do_increase(&mut c, line_idx, char_idx, &mut flashed);
            }
        }

        // set all flashed to 0
        for line_idx in 0..c.len() {
            for char_idx in 0..c[line_idx].len() {
                if flashed[line_idx][char_idx] {
                    c[line_idx][char_idx] = 0;
                    flashes += 1;
                }
            }
        }

        render_grid(c.clone());
    }

    println!("flashes={}", flashes);
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str
        .split("\n")
        .map(|s| {
            s.chars()
                .collect::<Vec<char>>()
                .iter()
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    render_grid(c.clone());

    let mut flashes = 0;

    for step in 1..=500 {
        let mut flashed = [[false; 10]; 10];
        // println!("STEP {}", step);

        let grid = c.clone();
        for line_idx in 0..grid.len() {
            for char_idx in 0..grid[line_idx].len() {
                c[line_idx][char_idx] += 1;
            }
        }

        for line_idx in 0..grid.len() {
            for char_idx in 0..grid[line_idx].len() {
                // println!("line_idx={}, char_idx={}", line_idx, char_idx);
                do_increase(&mut c, line_idx, char_idx, &mut flashed);
            }
        }

        // set all flashed to 0
        let mut all_flashed = true;
        for line_idx in 0..c.len() {
            for char_idx in 0..c[line_idx].len() {
                if flashed[line_idx][char_idx] {
                    c[line_idx][char_idx] = 0;
                    flashes += 1;
                } else {
                    all_flashed = false;
                }
            }
        }

        if all_flashed {
            println!("all flashed on step {}", step);
            return;
        }

        render_grid(c.clone());
    }
}

fn render_grid(grid: Vec<Vec<i32>>) {
    let should_render = false;

    if should_render {
        for line in grid {
            for chr in line {
                print!("{}", chr);
            }
            println!();
        }
        println!("--------------------------------");
    }
}

fn do_increase(
    grid: &mut Vec<Vec<i32>>,
    line_idx: usize,
    char_idx: usize,
    flashed: &mut [[bool; 10]; 10],
) {
    if grid[line_idx][char_idx] > 9 && !flashed[line_idx][char_idx] {
        flashed[line_idx][char_idx] = true;

        // up
        if line_idx > 0 {
            grid[line_idx - 1][char_idx] += 1;

            if grid[line_idx - 1][char_idx] > 9 {
                do_increase(grid, line_idx - 1, char_idx, flashed)
            }
        }

        // down
        if line_idx < grid.len() - 1 {
            grid[line_idx + 1][char_idx] += 1;

            if grid[line_idx + 1][char_idx] > 9 {
                do_increase(grid, line_idx + 1, char_idx, flashed)
            }
        }

        // left
        if char_idx > 0 {
            grid[line_idx][char_idx - 1] += 1;

            if grid[line_idx][char_idx - 1] > 9 {
                do_increase(grid, line_idx, char_idx - 1, flashed)
            }
        }

        // right
        if char_idx < grid[line_idx].len() - 1 {
            grid[line_idx][char_idx + 1] += 1;

            if grid[line_idx][char_idx + 1] > 9 {
                do_increase(grid, line_idx, char_idx + 1, flashed)
            }
        }

        // top left
        if line_idx > 0 && char_idx > 0 {
            grid[line_idx - 1][char_idx - 1] += 1;

            if grid[line_idx - 1][char_idx - 1] > 9 {
                do_increase(grid, line_idx - 1, char_idx - 1, flashed)
            }
        }

        // top right
        if line_idx > 0 && char_idx < grid[line_idx].len() - 1 {
            grid[line_idx - 1][char_idx + 1] += 1;

            if grid[line_idx - 1][char_idx + 1] > 9 {
                do_increase(grid, line_idx - 1, char_idx + 1, flashed)
            }
        }

        // bottom left
        if line_idx < grid.len() - 1 && char_idx > 0 {
            grid[line_idx + 1][char_idx - 1] += 1;

            if grid[line_idx + 1][char_idx - 1] > 9 {
                do_increase(grid, line_idx + 1, char_idx - 1, flashed)
            }
        }

        // bottom right
        if line_idx < grid.len() - 1 && char_idx < grid[line_idx].len() - 1 {
            grid[line_idx + 1][char_idx + 1] += 1;

            if grid[line_idx + 1][char_idx + 1] > 9 {
                do_increase(grid, line_idx + 1, char_idx + 1, flashed)
            }
        }
    }
}
