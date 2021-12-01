use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut last_depth: Option<u32> = None;
    let mut increase_count: u32 = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let new_depth = line.parse::<u32>().unwrap();
        if last_depth.is_some() && last_depth.unwrap() < new_depth {
            increase_count = increase_count + 1;
        }
        last_depth = Some(new_depth)
    }

    println!("{}", increase_count);
}
