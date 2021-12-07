use std::collections::LinkedList;
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
        assert_eq!(puzzle(), 1739);
    }
}

fn puzzle() -> u32 {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut last_values: LinkedList<u32> = LinkedList::new();
    let mut last_sum: Option<u32> = None;
    let mut increase_count: u32 = 0;
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        last_values.push_back(line.parse::<u32>().unwrap());
        if last_values.len() > 3 {
            last_values.pop_front();
        }

        if last_values.len() == 3 {
            let mut current_sum: u32 = 0;
            last_values.iter().for_each(|x| {
                current_sum = current_sum + x;
            });
            if last_sum.is_some() && last_sum.unwrap() < current_sum {
                increase_count = increase_count + 1;
            }
            last_sum = Some(current_sum);
        }
    }

    return increase_count;
}
