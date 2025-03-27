use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem014.txt")?;
    // let data = std::fs::read_to_string("test.txt")?;

    println!("Puzzle 14: Cyclops Chaos");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut grid = Vec::<Vec<i64>>::new();

    for line in &lines {
        grid.push(line.split(' ').map(|num| num.parse().unwrap()).collect());
    }

    let mut safest = i64::MAX;
    for i in 0..50 {
        let rowsum = grid[i].iter().sum::<i64>();
        let colsum = (0..50).map(|col| grid[i][col]).sum::<i64>();
        safest = safest.min(rowsum.min(colsum));
    }
    println!("  part 1 = {safest}");
    Ok(())
}
