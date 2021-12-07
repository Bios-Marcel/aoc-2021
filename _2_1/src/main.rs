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
        assert_eq!(puzzle(), 1499229);
    }
}

fn puzzle() -> u32 {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut horizontal_pos: u32 = 0;
    let mut depth: u32 = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let command: Vec<&str> = line.split(' ').collect();
        let amount = command[1].parse::<u32>().unwrap();
        match command[0] {
            "forward" => horizontal_pos = horizontal_pos + amount,
            "up" => depth = depth - amount,
            "down" => depth = depth + amount,
            _ => panic!(),
        }
    }

    return depth * horizontal_pos;
}
