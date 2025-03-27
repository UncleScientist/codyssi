use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem015.txt")?;
    // let data = std::fs::read_to_string("test.txt")?;

    println!("Puzzle 15: Games in a Storm");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let nums = lines
        .iter()
        .map(|line| {
            let (num, radix) = line.split_once(' ').unwrap();
            let radix = radix.parse().unwrap();
            from_str_radix(num, radix)
        })
        .collect::<Vec<_>>();

    println!("  part 1 = {}", nums.iter().max().unwrap());

    let sum = nums.iter().sum::<usize>();
    println!("  part 2 = {}", to_str_radix(sum, 68));

    println!("  part 3 = {}", smallest_base_for_4_digits(sum));
    println!(
        "           (or, mathematically, {})",
        f64::powf(sum as f64, 1. / 4.).ceil()
    );

    Ok(())
}

// log base x (num) = 4
//
// x^4 = num
//
fn smallest_base_for_4_digits(num: usize) -> usize {
    for base in 2..num {
        let mut start = num;
        for _ in 0..4 {
            start /= base;
        }
        if start == 0 {
            return base;
        }
    }

    panic!("ran out of numbers in usize");
}

fn to_str_radix(mut num: usize, radix: usize) -> String {
    const REST: [char; 6] = ['!', '@', '#', '$', '%', '^'];

    let mut result = Vec::new();
    while num > 0 {
        let digit = (num % radix) as u8;
        num /= radix;
        result.push(match digit {
            0..=9 => (digit + b'0') as char,
            10..=35 => ((digit - 10) + b'A') as char,
            36..=61 => ((digit - 36) + b'a') as char,
            _ => REST[digit as usize - 62],
        });
    }

    result.iter().rev().collect()
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
