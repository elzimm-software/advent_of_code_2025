use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[allow(unused_variables)]
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

    fn is_zero(&self) -> u32 {
        if self.n == 0 {
            1
        } else {
            0
        }
    }
}

impl Add<u32> for Dial {
    type Output = Dial;

    fn add(self, rhs: u32) -> Self::Output {
        Self::Output {
            n: (self.n + rhs) % 100,
        }
    }
}

impl AddAssign<u32> for Dial {
    fn add_assign(&mut self, rhs: u32) {
        self.n = (self.n + rhs) % 100;
    }
}

impl Sub<u32> for Dial {
    type Output = Dial;

    fn sub(self, rhs: u32) -> Self::Output {
        let rhs = rhs % 100;
        Self::Output {
            n: if rhs > self.n {
                100 - (rhs - self.n)
            } else {
                self.n - rhs
            },
        }
    }
}
impl SubAssign<u32> for Dial {
    fn sub_assign(&mut self, rhs: u32) {
        let rhs = rhs % 100;
        self.n = if rhs > self.n {
            100 - (rhs - self.n)
        } else {
            self.n - rhs
        }
    }
}


impl Display for Dial {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.n)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("inputs/d1.txt")?;
    let mut dial = Dial::new(50);

    let zeros = input.lines().fold(0, |acc, e| {
        match e.get(0..1).unwrap() {
            "R" => dial += e.get(1..).unwrap().parse::<u32>().unwrap(),
            "L" => dial -= e.get(1..).unwrap().parse::<u32>().unwrap(),
            _ => panic!(),
        }
        acc + dial.is_zero()
    });

    println!("{}", zeros);


    Ok(())
}