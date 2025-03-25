use std::{collections::HashMap, io::Error};

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem013.txt")?;

    println!("Puzzle 13: Windy Bargain");

    let sections = data
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut starting_balances = HashMap::<String, i64>::new();
    for line in sections[0].split('\n') {
        let words = line.split(' ').collect::<Vec<_>>();
        starting_balances.insert(words[0].into(), words[2].parse::<i64>().unwrap());
    }
    println!("{starting_balances:?}");

    let mut part1 = starting_balances.clone();
    for line in sections[1].split('\n') {
        if line.is_empty() {
            continue;
        }
        let words = line.split(' ').collect::<Vec<_>>();
        let amount = words[5].parse::<i64>().unwrap();
        *part1.get_mut(words[1].into()).unwrap() -= amount;
        *part1.get_mut(words[3].into()).unwrap() += amount;
    }
    let mut balances = part1.values().copied().collect::<Vec<_>>();
    balances.sort();
    println!("  part 1 = {}", balances.iter().rev().take(3).sum::<i64>());

    Ok(())
}
