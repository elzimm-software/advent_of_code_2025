use std::error::Error;
use std::fs::read_to_string;

#[allow(dead_code)]
const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

struct Dial {
    n: u32,
}

impl Dial {
    fn new(n: u32) -> Self {
        Self {
            n,
        }
    }

    fn right(&mut self, rotation: u32) -> u32 {
        let mut zeros = rotation / 100;
        let new_n = (self.n + rotation) % 100;
        if new_n < self.n {
            zeros += 1;
        }
        self.n = new_n;
        zeros
    }

    fn left(&mut self, rotation: u32) -> u32 {
        let mut zeros = rotation / 100;
        let rotation = rotation % 100;
        let new_n = if self.n < rotation {
            100 - (rotation - self.n)
        } else {
            self.n - rotation
        };
        if (new_n > self.n && self.n != 0)  || new_n == 0 {
            zeros += 1;
        }
        self.n = new_n;
        zeros
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut dial = Dial::new(50);

    let input = read_to_string("inputs/d1.txt")?;

    let zeros = input.lines().fold(0, |acc, e| {
        match e.get(0..1).unwrap() {
            "R" => acc + dial.right(e.get(1..).unwrap().parse::<u32>().unwrap()),
            "L" => acc + dial.left(e.get(1..).unwrap().parse::<u32>().unwrap()),
            _ => panic!(),
        }
    });

    println!("{zeros}");

    Ok(())
}