use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

//Required for array instantiation, since we copy the default value into each cell.
#[derive(Clone, Copy)]
struct Cell {
    val: u8,
    hit: bool,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.val)
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();

    reader
        .read_line(&mut buffer)
        .expect("error reading first line");
    let mut winning_numbers: Vec<u8> = Vec::new();
    buffer
        .split(',')
        .filter(|number| !number.is_empty())
        .map(|number| number.trim())
        //FIXME How do i get this to compile without adding a block, since
        //for_each expects a clojure that returns nothing.
        .for_each(|number: &str| {
            winning_numbers.push(number.parse::<u8>().unwrap());
        });

    let mut row = 0;
    let empty_cell = Cell { val: 0, hit: false };
    let mut current_board = [[empty_cell; 5]; 5];
    let mut boards: Vec<[[Cell; 5]; 5]> = Vec::new();

    //Skip empty line after winning numbers.
    reader.read_line(&mut buffer).expect("empty line expected");

    //Initially clear, since read_line appends.
    buffer.clear();
    while reader.read_line(&mut buffer).unwrap() > 0 {
        //Each board is seperate with an empty line
        if buffer.is_empty() || row == 5 {
            if row != 0 {
                &boards.push(current_board);
                current_board = [[empty_cell; 5]; 5];
                row = 0;
            }
            continue;
        }

        for (cell, element) in buffer.split_whitespace().enumerate() {
            current_board[row][cell] = Cell {
                val: element.parse::<u8>().unwrap(),
                hit: false,
            };
        }

        //Clear, since read_line appends.
        buffer.clear();
        row = row + 1;
    }

    //Last board gets lost otherwise, since the last line is EOF.
    if row != 0 {
        &boards.push(current_board);
    }

    for winning_number in winning_numbers {
        'boardloop: for board_index in 0..boards.len() {
            let board = &mut boards[board_index];
            for row_index in 0..5 {
                let row = &mut board[row_index];
                for column_index in 0..5 {
                    let cell = &mut row[column_index];
                    if cell.val == winning_number {
                        cell.hit = true;
                        if is_winning_board(&board) {
                            println!("{}", count_score(&board, winning_number));
                            //Got our board, therefore we stop program exectution.
                            return;
                        }

                        //A board can only contain a number once.
                        continue 'boardloop;
                    }
                }
            }
        }
    }
}

fn count_score(board: &[[Cell; 5]; 5], winning_number: u8) -> u32 {
    let mut sum_non_hit_cells: u32 = 0;
    for row_index in 0..5 {
        let row = &board[row_index];
        for column_index in 0..5 {
            let cell = &row[column_index];
            if !cell.hit {
                sum_non_hit_cells = sum_non_hit_cells + (cell.val as u32);
            }
        }
    }

    return sum_non_hit_cells * (winning_number as u32);
}

//FIXME Is there a shorter way to do this?
fn is_winning_board(board: &[[Cell; 5]; 5]) -> bool {
    //Horizontal
    for row_index in 0..5 {
        let row = &board[row_index];

        let mut row_won = true;
        for column_index in 0..5 {
            if !row[column_index].hit {
                row_won = false;
                break;
            }
        }

        if row_won {
            return true;
        }
    }

    //Vertical
    for column_index in 0..5 {
        let mut column_won = true;
        for row_index in 0..5 {
            if !board[row_index][column_index].hit {
                column_won = false;
                break;
            }
        }

        if column_won {
            return true;
        }
    }

    //Diagonal, top-left to bottom-right
    let mut top_left_to_bottom_right_win = true;
    for index in 0..5 {
        if !board[index][index].hit {
            top_left_to_bottom_right_win = false;
            break;
        }
    }

    if top_left_to_bottom_right_win {
        return true;
    }

    //Diagonal, bottom-left to top-right
    let mut bottom_left_to_top_right = true;
    let mut x = 0;
    let mut y = 5;
    loop {
        y = y - 1;
        if !board[y][x].hit {
            bottom_left_to_top_right = false;
            break;
        }

        if y == 0 {
            break;
        }
        x = x + 1;
    }

    bottom_left_to_top_right
}
