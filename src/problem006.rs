use std::{io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let lines = crate::read_sections(6, "Absurd Arithmetic")?;

    let ops = lines[0]
        .iter()
        .map(|line| line.parse::<PriceOp>().unwrap())
        .rev()
        .collect::<Vec<_>>();
    let price = |num: i64| ops.iter().fold(num, |num, op| op.apply(num));

    let mut nums = lines[1]
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();

    // Part 1
    let median = nums[nums.len() / 2];
    println!("  part 1 = {}", price(median));

    // Part 2
    let evens = nums.iter().filter(|num| *num % 2 == 0).sum::<i64>();
    println!("  part 2 = {}", price(evens));

    const MAX_PRICE: i64 = 15000000000000;
    println!(
        "  part 3 = {}",
        nums.iter()
            .filter(|num| price(**num) <= MAX_PRICE)
            .max()
            .unwrap()
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
