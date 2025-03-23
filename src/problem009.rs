use std::{io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem009.txt")?;

    println!("Puzzle 9: Patron Islands");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let points = lines
        .iter()
        .map(|line| line.parse::<Point>().unwrap())
        .collect::<Vec<_>>();

    let dists = points
        .iter()
        .map(|point| (point, point.dist_to_origin()))
        .collect::<Vec<_>>();

    let closest = dists.iter().min_by_key(|point| point.1).unwrap();
    let furthest = dists.iter().max_by_key(|point| point.1).unwrap();
    println!("  part 1 = {}", furthest.1 - closest.1);

    let closest_to_closest = points
        .iter()
        .filter(|point| *point != closest.0)
        .map(|point| (point, point.dist(closest.0)))
        .min_by_key(|(_, dist)| *dist)
        .unwrap();
    println!("  part 2 = {}", closest_to_closest.1);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn dist_to_origin(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn dist(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(", ").unwrap();
        let x = left[1..].parse::<i64>().unwrap();
        let y = right[..right.len() - 1].parse::<i64>().unwrap();
        Ok(Self { x, y })
    }
}
