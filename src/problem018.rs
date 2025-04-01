use std::{io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(18, "Crucial Crafting")?;

    let mut items = lines
        .iter()
        .map(|line| line.parse::<Item>().unwrap())
        .collect::<Vec<_>>();

    items.sort();

    println!(
        "  part 1 = {}",
        items
            .iter()
            .rev()
            .take(5)
            .map(|item| item.materials)
            .sum::<usize>()
    );

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Item {
    _id: usize,
    _code: String,
    quality: usize,
    cost: usize,
    materials: usize,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.quality == other.quality {
            self.cost.cmp(&other.cost)
        } else {
            self.quality.cmp(&other.quality)
        }
    }
}

impl FromStr for Item {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, right) = line.split_once(" | ").unwrap();

        let words = left.split(' ').collect::<Vec<_>>();
        let _id = words[0].parse().unwrap();
        let _code = words[1].to_string();

        let attrib = right.split(", ").collect::<Vec<_>>();
        let quality = attrib[0].split_once(" : ").unwrap().1.parse().unwrap();
        let cost = attrib[1].split_once(" : ").unwrap().1.parse().unwrap();
        let materials = attrib[2].split_once(" : ").unwrap().1.parse().unwrap();
        Ok(Self {
            _id,
            _code,
            quality,
            cost,
            materials,
        })
    }
}
