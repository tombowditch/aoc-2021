use crate::part2::part_two;
use std::fs;

mod part2;

fn main() {
    println!("===================== part one =====================");
    part_one();
    println!("===================== part two =====================");
    part_two();
}

fn part_one() {
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
    for input in bingo_input {
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
                if row_check {
                    println!("bingo in rows on board {}", i);
                }

                if col_check {
                    println!("bingo in columns on board {}", i);
                }

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

#[derive(Debug, Clone)]
enum BingoStatus {
    Picked(String),
    NotPicked(String),
}

fn check_rows(raw_board: Vec<Vec<BingoStatus>>) -> bool {
    for row in raw_board.iter() {
        // println!("calculated row: {:?}", row);

        let mut row_bingo = true;

        for item in row.iter() {
            if let BingoStatus::NotPicked(_) = item {
                row_bingo = false;
            }
        }

        if row_bingo {
            return true;
        }
    }

    return false;
}

fn check_columns(raw_board: Vec<Vec<BingoStatus>>) -> bool {
    let bit_size = raw_board[0].len();
    println!("bit size: {}", bit_size);

    for i in 0..bit_size {
        let mut column = vec![];
        for row in raw_board.iter() {
            column.push(row[i].clone());
        }

        // println!("calculated column: {:?}", column);

        let mut col_bingo = true;

        for item in column.iter() {
            if let BingoStatus::NotPicked(_) = item {
                col_bingo = false;
            }
        }

        if col_bingo {
            return true;
        }
    }

    return false;
}

mod tests {
    use super::*;

    #[test]
    fn test_check_cols_match() {
        let board: Vec<Vec<BingoStatus>> = vec![
            vec![
                BingoStatus::NotPicked("1".to_string()),
                BingoStatus::NotPicked("2".to_string()),
                BingoStatus::Picked("3".to_string()),
                BingoStatus::NotPicked("4".to_string()),
                BingoStatus::NotPicked("5".to_string()),
            ],
            vec![
                BingoStatus::NotPicked("6".to_string()),
                BingoStatus::NotPicked("7".to_string()),
                BingoStatus::Picked("8".to_string()),
                BingoStatus::NotPicked("9".to_string()),
                BingoStatus::NotPicked("10".to_string()),
            ],
            vec![
                BingoStatus::NotPicked("11".to_string()),
                BingoStatus::NotPicked("12".to_string()),
                BingoStatus::Picked("13".to_string()),
                BingoStatus::NotPicked("14".to_string()),
                BingoStatus::NotPicked("15".to_string()),
            ],
            vec![
                BingoStatus::NotPicked("16".to_string()),
                BingoStatus::NotPicked("17".to_string()),
                BingoStatus::Picked("18".to_string()),
                BingoStatus::NotPicked("19".to_string()),
                BingoStatus::NotPicked("20".to_string()),
            ],
        ];

        assert_eq!(true, check_columns(board.clone()));
    }

    #[test]
    fn test_check_cols_no_match() {
        let board: Vec<Vec<BingoStatus>> = vec![
            vec![
                BingoStatus::Picked("1".to_string()),
                BingoStatus::NotPicked("2".to_string()),
                BingoStatus::NotPicked("3".to_string()),
                BingoStatus::NotPicked("4".to_string()),
                BingoStatus::NotPicked("5".to_string()),
            ],
            vec![
                BingoStatus::NotPicked("6".to_string()),
                BingoStatus::NotPicked("7".to_string()),
                BingoStatus::Picked("8".to_string()),
                BingoStatus::Picked("9".to_string()),
                BingoStatus::NotPicked("10".to_string()),
            ],
            vec![
                BingoStatus::Picked("11".to_string()),
                BingoStatus::NotPicked("12".to_string()),
                BingoStatus::Picked("13".to_string()),
                BingoStatus::NotPicked("14".to_string()),
                BingoStatus::Picked("15".to_string()),
            ],
            vec![
                BingoStatus::NotPicked("16".to_string()),
                BingoStatus::Picked("17".to_string()),
                BingoStatus::Picked("18".to_string()),
                BingoStatus::NotPicked("19".to_string()),
                BingoStatus::NotPicked("20".to_string()),
            ],
        ];

        assert_eq!(false, check_columns(board.clone()));
    }

    #[test]
    fn test_check_rows_match() {
        let board: Vec<Vec<BingoStatus>> = vec![
            vec![
                BingoStatus::NotPicked("1".to_string()),
                BingoStatus::NotPicked("2".to_string()),
                BingoStatus::NotPicked("3".to_string()),
                BingoStatus::NotPicked("4".to_string()),
                BingoStatus::NotPicked("5".to_string()),
            ],
            vec![
                BingoStatus::NotPicked("6".to_string()),
                BingoStatus::NotPicked("7".to_string()),
                BingoStatus::NotPicked("8".to_string()),
                BingoStatus::NotPicked("9".to_string()),
                BingoStatus::NotPicked("10".to_string()),
            ],
            vec![
                BingoStatus::Picked("11".to_string()),
                BingoStatus::Picked("12".to_string()),
                BingoStatus::Picked("13".to_string()),
                BingoStatus::Picked("14".to_string()),
                BingoStatus::Picked("15".to_string()),
            ],
            vec![
                BingoStatus::NotPicked("16".to_string()),
                BingoStatus::NotPicked("17".to_string()),
                BingoStatus::Picked("18".to_string()),
                BingoStatus::NotPicked("19".to_string()),
                BingoStatus::NotPicked("20".to_string()),
            ],
        ];

        assert_eq!(true, check_rows(board.clone()));
    }
}
