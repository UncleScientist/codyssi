use std::{io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem007.txt")?;

    println!("Puzzle 7: Supplies in Surplus");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let boxen = lines
        .iter()
        .flat_map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            [
                left.parse::<BoxRange>().unwrap(),
                right.parse::<BoxRange>().unwrap(),
            ]
        })
        .collect::<Vec<_>>();
    println!(
        "  part 1 = {}",
        boxen.iter().map(|boxlist| boxlist.count()).sum::<i64>()
    );

    Ok(())
}

#[derive(Debug)]
struct BoxRange {
    min: i64,
    max: i64,
}

impl BoxRange {
    fn count(&self) -> i64 {
        self.max - self.min + 1
    }
}

impl FromStr for BoxRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((min, max)) = s.split_once('-') else {
            return Err(format!("Unable to split '{s}'"));
        };
        let Ok(min) = min.parse::<i64>() else {
            return Err(format!("Unable to parse '{min}'"));
        };
        let Ok(max) = max.parse::<i64>() else {
            return Err(format!("Unable to parse '{max}'"));
        };
        Ok(BoxRange { min, max })
    }
}
