use std::error::Error;
use std::fs::read_to_string;

#[allow(dead_code)]
const EXAMPLE_INPUT: &'static str = "987654321111111
811111111111119
234234234234278
818181911112111
";

fn max_joltage(banks: &str) -> u32 {
    let mut joltage = 0;
    for line in banks.lines() {
        let (max_first_index, max_first) = line
            .get(0..line.len()-1)
            .unwrap()
            .chars()
            .enumerate()
            .fold((0, 0), |acc, (i, ch)| {
                let n = ch.to_digit(10).unwrap();
                if n > acc.1 {
                    (i, n)
                } else {
                    acc
                }
            });
        let max_second = line
            .get(max_first_index+1..)
            .unwrap()
            .chars()
            .fold(0, |acc, ch| {
                let n = ch.to_digit(10).unwrap();
                if n > acc {
                    n
                } else {
                    acc
                }
            });
        joltage += max_first * 10 + max_second;
    }
    joltage
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("inputs/d3.txt")?;

    println!("{}", max_joltage(&input));

    Ok(())
}