use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem012.txt")?;

    println!("Puzzle 12: Risky Shortcut");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    println!(
        "  part 1 = {}",
        lines
            .iter()
            .map(|line| line.chars().filter(|ch| ch.is_alphabetic()).count())
            .sum::<usize>()
    );

    let part2 = lines
        .iter()
        .map(|line| reduce_by(line, reduce_with_hyphens))
        .collect::<Vec<_>>();
    println!(
        "  part 2 = {}",
        part2.iter().map(|line| line.len()).sum::<usize>()
    );

    let part3 = lines
        .iter()
        .map(|line| reduce_by(line, reduce_without_hyphens))
        .collect::<Vec<_>>();
    println!(
        "  part 3 = {}",
        part3.iter().map(|line| line.len()).sum::<usize>()
    );

    Ok(())
}

fn reduce_by<S: AsRef<str>>(s: S, test: fn(char, char) -> bool) -> String {
    let mut chars = s.as_ref().chars().collect::<Vec<_>>();
    let mut result = "".to_string();
    let mut removed = true;

    while removed {
        removed = false;
        result = "".to_string();
        for i in 0..chars.len() {
            if i + 1 < chars.len() {
                if test(chars[i], chars[i + 1]) {
                    result.push_str(&chars[i + 2..].iter().collect::<String>());
                    removed = true;
                    break;
                }
                result.push(chars[i]);
            } else {
                result.push(chars[i]);
            }
        }
        chars = result.chars().collect();
    }

    result
}

fn reduce_with_hyphens(ch1: char, ch2: char) -> bool {
    (ch1.is_numeric() && (ch2.is_alphabetic() || ch2 == '-'))
        || ((ch1.is_alphabetic() || ch1 == '-') && ch2.is_numeric())
}

fn reduce_without_hyphens(ch1: char, ch2: char) -> bool {
    (ch1.is_numeric() && ch2.is_alphabetic()) || (ch1.is_alphabetic() && ch2.is_numeric())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reduce_baa3() {
        assert_eq!("ba".to_string(), reduce_by("baa3", reduce_with_hyphens));
    }

    #[test]
    fn test_reduce_321ab() {
        assert_eq!("3".to_string(), reduce_by("321ab", reduce_with_hyphens));
    }

    #[test]
    fn test_reduce_a7b() {
        assert_eq!("b".to_string(), reduce_by("a7b", reduce_with_hyphens));
    }

    #[test]
    fn test_reduce_zdash4() {
        assert_eq!("z".to_string(), reduce_by("z-4", reduce_with_hyphens));
    }

    #[test]
    fn test_sample_input() {
        let t1 = reduce_by(
            "tv8cmj0i2951190z5w44fe205k542l5818ds05ib425h9lj260ud38-l6a06",
            reduce_with_hyphens,
        );
        let t2 = reduce_by(
            "a586m0eeuqqvt5-k-8434hb27ytha3i75-lw23-0cj856l7zn8234a05eron",
            reduce_with_hyphens,
        );
        assert_eq!(18, t1.len() + t2.len());
    }
}
