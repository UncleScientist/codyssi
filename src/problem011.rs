use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem011.txt")?;
    // let data = std::fs::read_to_string("test.txt")?;

    println!("Puzzle 11: Siren Disruption");

    let sections = data
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let freqs = sections[0]
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

    let mut part1 = freqs.clone();
    for swap in &swaps {
        part1.swap(swap.0, swap.1);
    }
    println!("  part 1 = {}", part1[test_index]);

    let mut part2 = freqs.clone();
    for i in 0..swaps.len() {
        let (x, y, z) = (swaps[i].0, swaps[i].1, swaps[(i + 1) % swaps.len()].0);
        let tmp = part2[z];
        part2[z] = part2[y];
        part2[y] = part2[x];
        part2[x] = tmp;
    }
    println!("  part 2 = {}", part2[test_index]);

    Ok(())
}
