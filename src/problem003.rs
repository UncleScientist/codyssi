use std::io::Error;

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(3, "Unformatted Readings")?;
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
    let mut num = pairs.iter().map(|p| p.0).sum::<usize>();
    println!("  part 2 = {num}");

    let num_to_char = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#"
        .chars()
        .collect::<Vec<_>>();
    let mut result = Vec::new();
    while num > 0 {
        let unit = num % 65;
        result.push(num_to_char[unit]);
        num /= 65;
    }

    println!("  part 3 = {}", result.iter().rev().collect::<String>());
    Ok(())
}
