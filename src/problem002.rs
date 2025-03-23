use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem002.txt")?;

    println!("Puzzle 2: Sensors and Circuits");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    println!(
        "  part 1 = {}",
        lines
            .iter()
            .enumerate()
            .filter_map(|(idx, val)| if *val == "TRUE" { Some(idx + 1) } else { None })
            .sum::<usize>()
    );

    Ok(())
}
