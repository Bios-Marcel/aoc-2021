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
        assert_eq!(puzzle(), 5410338);
    }
}

fn puzzle() -> u32 {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut values: Vec<u32> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let value = u32::from_str_radix(&line, 2).unwrap();
        values.push(value);
    }

    let mut oxygen_values = values.to_vec();
    filter_values(&mut oxygen_values, |ones_surplus| ones_surplus >= 0);

    let mut co2_values = values.to_vec();
    filter_values(&mut co2_values, |ones_surplus| ones_surplus < 0);

    return oxygen_values[0] * co2_values[0];
}

fn filter_values(values: &mut Vec<u32>, keep_set_bits_fn: fn(i32) -> bool) {
    for i in (0..12).rev() {
        //We can already have less than 2 values at iteration 8 for
        //example, so if we filter any further, we filter too much.
        if values.len() == 1 {
            break;
        }

        let mut ones_surplus: i32 = 0;
        let target_bit: u32 = 1 << i;
        for value in values.iter() {
            if value & target_bit == target_bit {
                ones_surplus = ones_surplus + 1;
            } else {
                ones_surplus = ones_surplus - 1;
            }
        }

        let keep_set_bits = keep_set_bits_fn(ones_surplus);
        let mut i = 0;
        while i < values.len() {
            let bit_set = values[i] & target_bit == target_bit;
            if bit_set != keep_set_bits {
                values.swap_remove(i);
                //Reaccess index, since we removed the current element and the
                //next element has the same index due to the swap.
                continue;
            }
            i = i + 1;
        }
    }
}
