use std::{collections::HashSet, io::Error};

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem004.txt")?;

    println!("Puzzle 4: Traversing the Country");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut locations = HashSet::new();
    for line in &lines {
        let (left, right) = line.split_once(" <-> ").unwrap();
        locations.insert(left);
        locations.insert(right);
    }
    println!("  part 1 = {}", locations.len());

    Ok(())
}
