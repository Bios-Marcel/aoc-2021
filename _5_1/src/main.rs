use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (x1, y1, x2, y2) = parse_line(line);

        //Vertical
        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                increment_cell(&mut matrix, x1, y);
            }
        }
        //Horizontal
        else if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                increment_cell(&mut matrix, x, y1);
            }
        }
        //Diagonal lines may exist but are to be ignored.
    }

    let mut overlapping_count: u32 = 0;
    for row in matrix.iter() {
        for value in row.iter() {
            if *value >= 2 {
                overlapping_count = overlapping_count + 1;
            }
        }
    }

    println!("{}", overlapping_count);
}
