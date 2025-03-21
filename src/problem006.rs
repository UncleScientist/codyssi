use std::{io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem006.txt")?;

    println!("Puzzle 6: Absurd Arithmetic");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let ops = lines[0..3]
        .iter()
        .map(|line| line.parse::<PriceOp>().unwrap())
        .rev()
        .collect::<Vec<_>>();
    println!("{ops:?}");

    let mut nums = lines[3..]
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();
    let median = nums[nums.len() / 2];
    println!(
        "  part 1 = {}",
        ops.iter().fold(median, |result, op| op.apply(result))
    );

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum PriceOp {
    Add(i64),
    Multiply(i64),
    Power(u32),
}

impl PriceOp {
    fn apply(&self, num: i64) -> i64 {
        match self {
            PriceOp::Add(a) => a + num,
            PriceOp::Multiply(m) => m * num,
            PriceOp::Power(p) => num.pow(*p),
        }
    }
}

impl FromStr for PriceOp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<_>>();
        let Ok(num) = words[words.len() - 1].parse::<i64>() else {
            return Err("Invalid number in '{s}'".into());
        };
        Ok(match words[2] {
            "ADD" => Self::Add(num),
            "MULTIPLY" => Self::Multiply(num),
            "RAISE" => Self::Power(num as u32),
            _ => return Err(format!("Invalid operation '{}'", words[2])),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_price_parse() {
        assert_eq!(PriceOp::Add(42), "Function A: ADD 42".parse().unwrap());
        assert_eq!(
            PriceOp::Multiply(142),
            "Function B: MULTIPLY 142".parse().unwrap()
        );
        assert_eq!(
            PriceOp::Power(12),
            "Function C: RAISE TO THE POWER OF 12".parse().unwrap()
        );
    }
}
