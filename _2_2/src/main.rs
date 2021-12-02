use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut horizontal_pos: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let command: Vec<&str> = line.split(' ').collect();
        let amount = command[1].parse::<u32>().unwrap();
        match command[0] {
            "forward" => {
                horizontal_pos = horizontal_pos + amount;
                depth = depth + aim * amount;
            },
            "up" => aim = aim - amount,
            "down" => aim = aim + amount,
            _ => panic!(),
        }
    }

    println!("{}", depth * horizontal_pos);
}
