use std::io::Error;

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(10, "Lotus Scramble")?;

    println!(
        "  part 1 = {}",
        lines[0].chars().filter(|ch| ch.is_alphabetic()).count()
    );

    let conversion = |ch| match ch {
        'a'..='z' => Some((ch as u8 - b'a') as i64 + 1),
        'A'..='Z' => Some((ch as u8 - b'A') as i64 + 27),
        _ => None,
    };

    println!(
        "  part 2 = {}",
        lines[0].chars().filter_map(conversion).sum::<i64>()
    );

    let mut total = 0;
    let mut prev = 0;
    for ch in lines[0].chars() {
        if let Some(val) = conversion(ch) {
            total += val;
            prev = val;
        } else {
            prev = (prev * 2) - 5;
            while prev < 1 {
                prev += 52;
            }
            while prev > 52 {
                prev -= 52;
            }
            total += prev;
        }
    }
    println!("  part 3 = {total}");

    Ok(())
}
