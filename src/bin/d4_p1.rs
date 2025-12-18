use std::error::Error;
use std::fs::read_to_string;
use std::str::Chars;

#[allow(dead_code)]
const EXAMPLE_INPUT: &'static str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

fn row_count(mut row: Chars) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::with_capacity(row.size_hint().1.unwrap_or_else(|| row.clone().count()));
    let mut prev = '.';
    let mut this = row.next().unwrap();
    while let Some(next) = row.next() {
        vec.push((prev == '@') as u8 + (this == '@') as u8 + (next == '@') as u8);
        prev = this;
        this = next;
    }
    vec.push((prev == '@') as u8 + (this == '@') as u8);
    vec
}

fn check_row(this: &Vec<u8>, this_ch: &Vec<char>, prev: &Vec<u8>, next: &Vec<u8>) -> u32 {
    let mut count: u32 = 0;
    for i in 0..this.len() {
        if this_ch[i] != '.' && (prev[i] + this[i]+next[i]) <= 4 {
            count += 1;
        }
    }
    count
}

fn forkliftable_rolls(grid: &str) -> u32 {
    let mut grid = grid.lines();
    let this_ch: Chars = grid.next().unwrap().chars();
    let mut this: Vec<u8> = row_count(this_ch.clone());
    let mut this_ch_vec: Vec<char> = this_ch.collect();
    let mut prev: Vec<u8> = vec![0; this.len()];
    let mut acc = 0;
    for line in grid {
        let next = row_count(line.chars());
        acc += check_row(&this, &this_ch_vec, &prev, &next);
        prev = this;
        this_ch_vec = line.chars().collect();
        this = next;
    }
    let next = vec![0; this.len()];

    acc + check_row(&this, &this_ch_vec, &prev, &next)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("inputs/d4.txt")?;

    println!("{}", forkliftable_rolls(&input));

    Ok(())
}
