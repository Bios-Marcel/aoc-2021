use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut values: Vec<u32> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let value = u32::from_str_radix(&line, 2).unwrap();
        values.push(value);
    }

    let mut gamma: u32 = 0;

    //Each line of the puzzle input has 12 bits, so we shift at max 11 to the left.
    for i in 0..11 {
        let mut ones_surplus: i32 = 0;
        let target_bit: u32 = 1 << i;
        for value in values.iter() {
            if value & target_bit == target_bit {
                ones_surplus = ones_surplus + 1;
            } else {
                ones_surplus = ones_surplus - 1;
            }
        }

        if ones_surplus > 0 {
            gamma |= target_bit
        } else {
            //Since we have more than the amount of bits used by the
            //puzzle (12), we flip all the bits and only take the first 12.
            gamma &= !target_bit & 0b1111_1111_1111
        }
    }

    //epsilon is gamma flipped and gamma*epsilon is power.
    //Since we have more than the amount of bits used by the
    //puzzle (12), we flip all the bits and only take the first 12.
    println!("{}", gamma * (!gamma & 0b1111_1111_1111));
}
