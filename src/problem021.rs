use std::{collections::HashMap, io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let sections = crate::read_sections(21, "Spiralling Stairs")?;

    let stairs = sections[0]
        .iter()
        .map(|stair| stair.parse::<Stair>().unwrap())
        .collect::<Vec<_>>();
    let (_, magnitudes) = sections[1][0].split_once(" : ").unwrap();
    let magnitudes = magnitudes
        .split(", ")
        .map(|num| num.parse::<u128>().unwrap())
        .collect::<Vec<_>>();

    let mut ways = CountWays::default();
    println!(
        "  part 1 = {}",
        ways.count(stairs[0].to - stairs[0].from, &magnitudes)
    );
    Ok(())
}

#[derive(Debug, Default)]
struct CountWays {
    cache: HashMap<u128, u128>, // maps remaining steps to answer for that number of steps
}

impl CountWays {
    fn count(&mut self, remaining: u128, mags: &[u128]) -> u128 {
        if let Some(result) = self.cache.get(&remaining) {
            return *result;
        }
        let mut total = 0;
        for mag in mags {
            if *mag == remaining {
                total += 1;
            } else if *mag < remaining {
                total += self.count(remaining - *mag, mags);
            }
        }
        self.cache.insert(remaining, total);
        total
    }
}

#[derive(Debug)]
struct Stair {
    _id: usize,
    from: u128,
    to: u128,
    _start: StairLoc,
    _end: StairLoc,
}

#[derive(Debug)]
enum StairLoc {
    Start,
    End,
    Stair(usize),
}

impl FromStr for Stair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" : ").collect::<Vec<_>>();
        let id = parts[0][1..].parse::<usize>().unwrap();
        let (from, to) = parts[1].split_once(" -> ").unwrap();
        let start_end = parts[2].split(' ').collect::<Vec<_>>();
        Ok(Stair {
            _id: id,
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
            _start: start_end[1].parse::<StairLoc>().unwrap(),
            _end: start_end[3].parse::<StairLoc>().unwrap(),
        })
    }
}
impl FromStr for StairLoc {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "START" => Self::Start,
            "END" => Self::End,
            _ => Self::Stair(s[1..].parse::<usize>().unwrap()),
        })
    }
}
