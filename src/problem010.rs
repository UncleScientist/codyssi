use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem010.txt")?;
    // let data = std::fs::read_to_string("test.txt")?;

    println!("Puzzle 10: Lotus Scramble");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!(
        "  part 1 = {}",
        lines[0].chars().filter(|ch| ch.is_alphabetic()).count()
    );

    println!(
        "  part 2 = {}",
        lines[0]
            .chars()
            .map(|ch| match ch {
                'a'..='z' => (ch as u8 - b'a') as usize + 1,
                'A'..='Z' => (ch as u8 - b'A') as usize + 27,
                _ => 0,
            })
            .sum::<usize>()
    );

    Ok(())
}
