use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem015.txt")?;
    // let data = std::fs::read_to_string("test.txt")?;

    println!("Puzzle 15: Games in a Storm");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    println!(
        "  part 1 = {}",
        lines
            .iter()
            .map(|line| {
                let (num, radix) = line.split_once(' ').unwrap();
                let radix = radix.parse().unwrap();
                from_str_radix(num, radix)
            })
            .max()
            .unwrap()
    );

    Ok(())
}

fn from_str_radix<S: AsRef<str>>(s: S, radix: usize) -> usize {
    s.as_ref().chars().fold(0usize, |cur, ch| {
        cur * radix
            + match ch {
                '0'..='9' => (ch as u8 - b'0') as usize,
                'A'..='Z' => (ch as u8 - b'A') as usize + 10,
                'a'..='z' => (ch as u8 - b'a') as usize + 36,
                _ => panic!("Invalid digit {ch}"),
            }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base_10() {
        assert_eq!(123, from_str_radix("123", 10));
    }

    #[test]
    fn test_base_16() {
        assert_eq!(4011, from_str_radix("FAB", 16));
    }

    #[test]
    fn test_base_61() {
        assert_eq!(2375258103548, from_str_radix("k6IHxTD", 61));
    }
}
