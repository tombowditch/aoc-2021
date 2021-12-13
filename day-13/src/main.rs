use std::fs;

fn main() {
    part_one();
    part_two();
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

// ...#... 0
// ..#.... 1 ----
// .....#. 2
// ------- 3 ---
// .#..... 4
// .....#. 5  ---
// ....#.. 6

// ========

// ...##..
// ..#..#.
// .#...#.

impl Fold {
    fn fold_board(&self, board: &mut Vec<(usize, usize)>) {
        println!("fold {:?}", self);
        let mut new_board: Vec<(usize, usize)> = vec![];

        match self {
            Fold::X(c) => {
                for (x, y) in board.clone() {
                    if x < *c {
                        new_board.push((x, y));
                        continue;
                    }

                    let new_coord = ((c - (x - c)), y);
                    new_board.push(new_coord);
                }
            }
            Fold::Y(c) => {
                for (x, y) in board.clone() {
                    if y < *c {
                        new_board.push((x, y));
                        continue;
                    }

                    let new_coord = (x, (c - (y - c)));
                    new_board.push(new_coord);
                }
            }
        }
        new_board.sort();
        new_board.dedup();

        *board = new_board;
    }
}

fn part_one() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut folds: Vec<Fold> = vec![];
    let c = input_str.split("\n").collect::<Vec<_>>();

    let mut board = vec![];

    for fold in c.iter().filter(|f| f.contains("fold along")) {
        let fsplit = fold.split(" ").collect::<Vec<_>>()[2]
            .split("=")
            .collect::<Vec<_>>();

        match fsplit[0] {
            "x" => folds.push(Fold::X(fsplit[1].parse::<usize>().unwrap())),
            "y" => folds.push(Fold::Y(fsplit[1].parse::<usize>().unwrap())),
            _ => panic!("Unknown fold"),
        }
    }

    for rawline in c {
        let linesplit = rawline.split(",").collect::<Vec<_>>();
        if linesplit.len() != 2 {
            continue;
        }

        let char_idx = linesplit[0].parse::<usize>().unwrap();
        let line_idx = linesplit[1].parse::<usize>().unwrap();

        board.push((char_idx, line_idx));
    }

    println!("{:?}", board);
    println!("fold instructions: {:?}", folds);

    folds[0].fold_board(&mut board);

    println!("{:?}", board);

    println!("{:?}", board.len());
}

fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut folds: Vec<Fold> = vec![];
    let c = input_str.split("\n").collect::<Vec<_>>();

    let mut board = vec![];

    for fold in c.iter().filter(|f| f.contains("fold along")) {
        let fsplit = fold.split(" ").collect::<Vec<_>>()[2]
            .split("=")
            .collect::<Vec<_>>();

        match fsplit[0] {
            "x" => folds.push(Fold::X(fsplit[1].parse::<usize>().unwrap())),
            "y" => folds.push(Fold::Y(fsplit[1].parse::<usize>().unwrap())),
            _ => panic!("Unknown fold"),
        }
    }

    for rawline in c {
        let linesplit = rawline.split(",").collect::<Vec<_>>();
        if linesplit.len() != 2 {
            continue;
        }

        let char_idx = linesplit[0].parse::<usize>().unwrap();
        let line_idx = linesplit[1].parse::<usize>().unwrap();

        board.push((char_idx, line_idx));
    }

    println!("{:?}", board);
    println!("fold instructions: {:?}", folds);

    for fold in folds {
        fold.fold_board(&mut board);
    }

    println!("{:?}", board);

    println!("{:?}", board.len());

    render_board(&board);
}

fn render_board(board: &[(usize, usize)]) {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for (x, y) in board {
        xs.push(x);
        ys.push(y);
    }

    xs.sort();
    ys.sort();

    let max_x = xs.last().unwrap();
    let max_y = ys.last().unwrap();

    for y in 0..=**max_y {
        for x in 0..=**max_x {
            if board.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}
