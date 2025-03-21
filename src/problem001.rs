use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem001.txt")?;

    println!("Puzzle 1: Handling the Budget");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut prices = lines
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    prices.sort();
    println!("  part 1 = {}", prices.iter().sum::<usize>());
    println!("  part 2 = {}", prices.iter().rev().skip(20).sum::<usize>());

    Ok(())
}
