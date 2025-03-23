use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem003.txt")?;

    println!("Puzzle 3: Unformatted Readings");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let pairs = lines
        .iter()
        .map(|line| {
            let (strval, base) = line.split_once(' ').unwrap();
            let base = base.parse::<u32>().unwrap();
            let num = usize::from_str_radix(strval, base).unwrap();
            (num, base)
        })
        .collect::<Vec<_>>();

    println!("  part 1 = {}", pairs.iter().map(|p| p.1).sum::<u32>());
    println!("  part 2 = {}", pairs.iter().map(|p| p.0).sum::<usize>());
    Ok(())
}
