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

    println!(
        "  part 2 = {}",
        boxen
            .chunks(2)
            .map(|pair| if pair[0].overlaps(&pair[1]) {
                pair[0].merge(&pair[1]).count()
            } else {
                pair[0].count() + pair[1].count()
            })
            .sum::<i64>()
    );

    println!(
        "  part 3 = {}",
        boxen
            .windows(4)
            .enumerate()
            .filter_map(|(idx, set)| if idx % 2 == 0 { Some(set) } else { None })
            .map(merge_and_sum)
            .max()
            .unwrap()
    );

    Ok(())
}

fn merge_and_sum(list: &[BoxRange]) -> i64 {
    let mut boxlist: Vec<BoxRange> = list.to_vec();
    let mut idx = 0;
    while idx < boxlist.len() - 1 {
        let mut merged = false;
        for next in idx + 1..boxlist.len() {
            if boxlist[idx].overlaps(&boxlist[next]) {
                boxlist[idx] = boxlist[idx].merge(&boxlist[next]);
                boxlist.remove(next);
                merged = true;
                break;
            }
        }
        if !merged {
            idx += 1;
        }
    }
    boxlist.iter().map(|b| b.count()).sum()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct BoxRange {
    min: i64,
    max: i64,
}

impl BoxRange {
    fn count(&self) -> i64 {
        self.max - self.min + 1
    }

    fn overlaps(&self, other: &Self) -> bool {
        let smaller_max = self.max.min(other.max);
        let larger_min = self.min.max(other.min);
        larger_min <= smaller_max
    }

    fn merge(&self, other: &Self) -> Self {
        let min = self.min.min(other.min);
        let max = self.max.max(other.max);
        Self { min, max }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_overlapping_ranges() {
        let b1: BoxRange = "8-9".parse().unwrap();
        let b2: BoxRange = "9-10".parse().unwrap();
        assert!(b1.overlaps(&b2));
    }

    #[test]
    fn test_overlaps_completely() {
        let b1: BoxRange = "8-9".parse().unwrap();
        let b2: BoxRange = "4-20".parse().unwrap();
        assert!(b1.overlaps(&b2));
    }

    #[test]
    fn test_merge() {
        let b1: BoxRange = "4-9".parse().unwrap();
        let b2: BoxRange = "8-20".parse().unwrap();
        assert_eq!(BoxRange { min: 4, max: 20 }, b1.merge(&b2));
    }
}
