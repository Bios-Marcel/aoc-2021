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
        assert_eq!(puzzle(), 349549);
    }
}

fn puzzle() -> usize {
    let file = File::open("input").unwrap();
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .expect("error reading first line");
    let mut lanternfish_ages: Vec<u8> = Vec::new();
    buffer
        .split(',')
        .filter(|number| !number.is_empty())
        .map(|number| number.trim())
        .for_each(|number: &str| lanternfish_ages.push(number.parse::<u8>().unwrap()));

    for _ in 0..80 {
        for index in 0..lanternfish_ages.len() {
            let age = lanternfish_ages[index];
            if age == 0 {
                lanternfish_ages[index] = 6;
                lanternfish_ages.push(8);
            } else {
                lanternfish_ages[index] = age - 1;
            }
        }
    }

    lanternfish_ages.len()
}
