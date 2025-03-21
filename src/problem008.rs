use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem008.txt")?;

    println!("Puzzle 8: Aeolian Transmissions");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!("  part 1 = {}", lines.iter().map(units).sum::<isize>());
    println!(
        "  part 2 = {}",
        lines.iter().map(compress).map(units).sum::<isize>()
    );

    Ok(())
}

fn units<S: AsRef<str>>(s: S) -> isize {
    s.as_ref()
        .chars()
        .map(|ch| {
            if ch.is_alphabetic() {
                (ch as u8 - b'A') as isize + 1
            } else if ch.is_numeric() {
                (ch as u8 - b'0') as isize
            } else {
                panic!("Invalid char '{ch}'");
            }
        })
        .sum::<isize>()
}

fn compress<S: AsRef<str>>(s: S) -> String {
    let keep = s.as_ref().len() / 10;
    format!(
        "{}{}{}",
        &s.as_ref()[0..keep],
        s.as_ref().len() - keep * 2,
        &s.as_ref()[s.as_ref().len() - keep..]
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compression_10() {
        assert_eq!("A8J".to_string(), compress("ABCDEFGHIJ"));
    }

    #[test]
    fn test_compression_20() {
        assert_eq!("NN20MM".to_string(), compress("NNBUSSSSSDSSZZZZMMMMMMMM"));
    }
}
