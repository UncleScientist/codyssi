use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem011.txt")?;
    // let data = std::fs::read_to_string("test.txt")?;

    println!("Puzzle 11: Siren Disruption");

    let sections = data
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let mut freqs = sections[0]
        .split('\n')
        .map(|freq| freq.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let swaps = sections[1]
        .split('\n')
        .map(|swap| {
            let (left, right) = swap.split_once('-').unwrap();
            (
                left.parse::<usize>().unwrap() - 1,
                right.parse::<usize>().unwrap() - 1,
            )
        })
        .collect::<Vec<_>>();
    let test_index = sections[2].trim().parse::<usize>().unwrap() - 1;

    for swap in &swaps {
        freqs.swap(swap.0, swap.1);
    }
    println!("  part 1 = {}", freqs[test_index]);

    Ok(())
}
