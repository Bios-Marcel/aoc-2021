use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", puzzle());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(puzzle(), 22083);
    }
}

fn puzzle() -> u32 {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<u8>> = Vec::new();
    let increase: fn(usize) -> usize = |n| n + 1;
    let decrease: fn(usize) -> usize = |n| n - 1;
    let keep: fn(usize) -> usize = |n| n;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (x1, y1, x2, y2) = parse_line(line);

        let x_upd: fn(usize) -> usize;
        if x1 < x2 {
            x_upd = increase;
        } else if x1 > x2 {
            x_upd = decrease;
        } else {
            x_upd = keep;
        }

        let y_upd: fn(usize) -> usize;
        if y1 < y2 {
            y_upd = increase;
        } else if y1 > y2 {
            y_upd = decrease;
        } else {
            y_upd = keep;
        }

        let mut x = x1;
        let mut y = y1;
        loop {
            //A While-loop wouldn't work, since a line like 8,8->8,8 would never even iterate once.
            //Hence we check whether the exit condition is reached at the start and then do our logic.
            let exit_condition_reached = x == x2 && y == y2;

            increment_cell(&mut matrix, x, y);
            x = x_upd(x);
            y = y_upd(y);

            if exit_condition_reached {
                break;
            }
        }
    }

    let mut overlapping_count: u32 = 0;
    for row in matrix.iter() {
        for value in row.iter() {
            if *value >= 2 {
                overlapping_count = overlapping_count + 1;
            }
        }
    }

    return overlapping_count;
}

//"x1,y1 -> x2->y1" -> (x1,y1,x2,y2)
fn parse_line(text: String) -> (usize, usize, usize, usize) {
    let coords: Vec<Vec<usize>> = text
        .split(" -> ")
        .map(|coordinate_pair| {
            coordinate_pair
                .split(',')
                .map(|coordinate| coordinate.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    return (coords[0][0], coords[0][1], coords[1][0], coords[1][1]);
}

fn increment_cell(matrix: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    if matrix.len() <= y {
        matrix.resize_with(y + 1, || Vec::new());
    }

    let row = &mut matrix[y];
    if row.len() <= x {
        row.resize(x + 1, 0);
    }

    row[x] = row[x] + 1;
}
