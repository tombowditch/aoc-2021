use std::fs;

use crate::{check_columns, check_rows, BingoStatus};

pub(crate) fn part_two() {
    let input_str = fs::read_to_string("input.txt").unwrap();
    let mut c = input_str.split("\n").collect::<Vec<&str>>();

    let bingo_input = c.clone()[0].split(",").collect::<Vec<&str>>();
    println!("bingo input: {:?}", bingo_input);

    c = c[2..].to_vec();

    let mut raw_boards: Vec<Vec<Vec<BingoStatus>>> = vec![];

    let mut tmp_board: Vec<Vec<BingoStatus>> = vec![];
    for line in c.clone() {
        if line == "" && tmp_board.len() > 0 {
            // reset tmp_board
            raw_boards.push(tmp_board);
            tmp_board = vec![];
            continue;
        }

        tmp_board.push(
            line.split_whitespace()
                .map(|s| BingoStatus::NotPicked(s.to_string()))
                .collect::<Vec<BingoStatus>>(),
        );
    }
    raw_boards.push(tmp_board);

    println!("raw boards structure = {:#?}", raw_boards);

    let mut bingo = false;
    let mut bingod_boards: Vec<usize> = vec![];

    for input in bingo_input {
        println!("bingoed boards: {:?}", bingod_boards);
        println!("input: {}", input);
        if bingo {
            return;
        }
        let new_boards = raw_boards
            .iter()
            .map(|board| {
                board
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|item| {
                                if let BingoStatus::NotPicked(inner) = item {
                                    if inner == input {
                                        return BingoStatus::Picked(inner.clone());
                                    }
                                }

                                return item.clone();
                            })
                            .collect::<Vec<BingoStatus>>()
                    })
                    .collect::<Vec<Vec<BingoStatus>>>()
            })
            .collect::<Vec<Vec<Vec<BingoStatus>>>>();
        raw_boards = new_boards;

        // do the check
        for (mut i, board) in raw_boards.clone().iter().enumerate() {
            i += 1;
            let row_check = check_rows(board.clone());
            let col_check = check_columns(board.clone());

            // println!("checking board {} = {:#?}", i, board);

            if row_check || col_check {
                println!("bingo on board {}", i);
            }
            if (row_check || col_check) && !bingod_boards.contains(&i) {
                bingod_boards.push(i);
                if row_check {
                    println!("bingo in rows on board {}", i);
                }

                if col_check {
                    println!("bingo in columns on board {}", i);
                }

                let bingo_boards = raw_boards.len();
                println!(
                    "--------- bingod {}, tot boards {}",
                    bingod_boards.len(),
                    bingo_boards
                );
                if bingod_boards.len() == bingo_boards {
                    // sum Unpicked
                    let last_pick = input.parse::<i32>().unwrap();
                    let mut unpicked_sum = 0;

                    board.iter().for_each(|row| {
                        row.iter().for_each(|item| {
                            if let BingoStatus::NotPicked(inner) = item {
                                unpicked_sum += inner.parse::<i32>().unwrap();
                            }
                        })
                    });

                    println!("unpicked sum: {}", unpicked_sum);
                    println!("last pick: {}", last_pick);

                    println!("result {}", unpicked_sum * last_pick);

                    bingo = true;
                    return;
                }
            }
        }
    }
}
