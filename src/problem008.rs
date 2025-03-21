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
    println!(
        "  part 3 = {}",
        lines.iter().map(rle).map(units).sum::<isize>()
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
    let len = s.as_ref().len();
    let keep = len / 10;
    format!(
        "{}{}{}",
        &s.as_ref()[0..keep],
        len - keep * 2,
        &s.as_ref()[len - keep..]
    )
}

fn rle<S: AsRef<str>>(s: S) -> String {
    let mut result = String::from("");
    if !s.as_ref().is_empty() {
        let mut cur = None;
        let mut count = 0;
        for ch in s.as_ref().chars() {
            if cur == Some(ch) {
                count += 1;
                continue;
            } else if let Some(c) = cur {
                result.push_str(&format!("{count}"));
                result.push(c);
            }
            cur = Some(ch);
            count = 1;
        }
        result.push_str(&format!("{count}"));
        result.push(cur.unwrap());
    }
    result
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

    #[test]
    fn test_rle() {
        assert_eq!(
            "2N1B1U5S1D2S4Z8M".to_string(),
            rle("NNBUSSSSSDSSZZZZMMMMMMMM")
        );
    }
}
