use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem003.txt")?;

    println!("Puzzle 3: Unformatted Readings");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!(
        "  part 1 = {}",
        lines
            .iter()
            .map(|line| {
                let (_, base) = line.split_once(' ').unwrap();
                base.parse::<usize>().unwrap()
            })
            .sum::<usize>()
    );
    Ok(())
}
