use std::error::Error;
use std::fs::read_to_string;

#[allow(dead_code)]
const EXAMPLE_INPUT: &'static str = "987654321111111
811111111111119
234234234234278
818181911112111";

fn max_joltage(n: usize, banks: &str) -> u64 {
    let mut joltage = 0u64;

    for line in banks.lines() {
        let mut bank_max = 0u64;
        let len = line.len();
        let mut last_index = -1isize;
        for i in (1..=n).rev() {
            let start_index = (last_index + 1) as usize;
            let end_index = start_index + len - (start_index + i);
            let search_area = line.get(start_index..=end_index).unwrap();
            let (local_max_index, local_max) =
                search_area
                    .chars()
                    .enumerate()
                    .fold((0, 0), |acc, (j, ch)| {
                        let val = ch.to_digit(10).unwrap();
                        if val > acc.1 { (j, val) } else { acc }
                    });
            last_index += 1 + local_max_index as isize;
            bank_max += local_max as u64 * 10u64.pow((i - 1) as u32);
        }
        joltage += bank_max;
    }

    joltage
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("inputs/d3.txt")?;

    println!("{}", max_joltage(12, &input));

    Ok(())
}
