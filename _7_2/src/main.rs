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
        assert_eq!(puzzle(), 90040997);
    }
}

fn puzzle() -> isize {
    let file = File::open("input").unwrap();
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .expect("error reading first line");
    let crab_positions = buffer
        .split(',')
        .filter(|number| !number.is_empty())
        .map(|number| number.trim())
        .map(|number: &str| number.parse().unwrap())
        .collect::<Vec<isize>>();

    let mut fuel_required = isize::MAX;
    for pos_a in *crab_positions.iter().min().unwrap()..=*crab_positions.iter().max().unwrap() {
        let mut fuel_required_temp = 0;
        for pos_b in crab_positions.iter() {
            fuel_required_temp =
                fuel_required_temp + calculate_required_fuel((pos_a - pos_b).abs());
        }

        if fuel_required_temp < fuel_required {
            fuel_required = fuel_required_temp
        }
    }

    fuel_required
}

fn calculate_required_fuel(distance: isize) -> isize {
    distance * (distance + 1) / 2
}
