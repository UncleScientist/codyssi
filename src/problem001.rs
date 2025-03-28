use std::io::Error;

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(1, "Handling the Budget")?;

    let mut prices = lines
        .iter()
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let part3 = prices
        .iter()
        .enumerate()
        .map(|(idx, price)| if idx % 2 == 0 { *price } else { -*price })
        .sum::<isize>();

    println!("  part 1 = {}", prices.iter().sum::<isize>());
    prices.sort();
    println!("  part 2 = {}", prices.iter().rev().skip(20).sum::<isize>());
    println!("  part 3 = {part3}");

    Ok(())
}
