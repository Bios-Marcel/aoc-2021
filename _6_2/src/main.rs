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
        assert_eq!(puzzle(), 1589590444365);
    }
}

fn puzzle() -> usize {
    let file = File::open("input").unwrap();
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .expect("error reading first line");
    let mut lanternfish_ages: Vec<usize> = Vec::new();
    buffer
        .split(',')
        .filter(|number| !number.is_empty())
        .map(|number| number.trim())
        .for_each(|number: &str| lanternfish_ages.push(number.parse::<usize>().unwrap()));

    //No more list approach, as it's too slow for big amounts of data.

    //age-1 is the index;
    let mut count_by_timer: [usize; 9] = [0; 9];

    //Initial timer state
    for timer in lanternfish_ages.iter() {
        count_by_timer[*timer] = count_by_timer[*timer] + 1;
    }

    for _ in 0..256 {
        let new_born_count = count_by_timer[0];
        for timer in 0..8 {
            count_by_timer[timer] = count_by_timer[timer + 1];
        }
        count_by_timer[8] = new_born_count;
        count_by_timer[6] = count_by_timer[6] + new_born_count;
    }

    let mut fish_count = 0;
    for count in count_by_timer {
        fish_count = fish_count + count;
    }
    fish_count
}
