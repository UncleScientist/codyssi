use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem012.txt")?;

    println!("Puzzle 12: Risky Shortcut");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    println!(
        "  part 1 = {}",
        lines
            .iter()
            .map(|line| line.chars().filter(|ch| ch.is_alphabetic()).count())
            .sum::<usize>()
    );

    Ok(())
}
