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

    let part2 = lines.iter().map(reduce).collect::<Vec<_>>();
    println!(
        "  part 2 = {}",
        part2.iter().map(|line| line.len()).sum::<usize>()
    );

    Ok(())
}

fn reduce<S: AsRef<str>>(s: S) -> String {
    let mut chars = s.as_ref().chars().collect::<Vec<_>>();
    let mut result = "".to_string();
    let mut removed = true;

    while removed {
        removed = false;
        result = "".to_string();
        for i in 0..chars.len() {
            if i + 1 < chars.len() {
                if (chars[i].is_numeric() && (chars[i + 1].is_alphabetic() || chars[i + 1] == '-'))
                    || ((chars[i].is_alphabetic() || chars[i] == '-') && chars[i + 1].is_numeric())
                {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reduce_baa3() {
        assert_eq!("ba".to_string(), reduce("baa3"));
    }

    #[test]
    fn test_reduce_321ab() {
        assert_eq!("3".to_string(), reduce("321ab"));
    }

    #[test]
    fn test_reduce_a7b() {
        assert_eq!("b".to_string(), reduce("a7b"));
    }

    #[test]
    fn test_reduce_zdash4() {
        assert_eq!("z".to_string(), reduce("z-4"));
    }

    #[test]
    fn test_sample_input() {
        let t1 = reduce("tv8cmj0i2951190z5w44fe205k542l5818ds05ib425h9lj260ud38-l6a06");
        let t2 = reduce("a586m0eeuqqvt5-k-8434hb27ytha3i75-lw23-0cj856l7zn8234a05eron");
        assert_eq!(18, t1.len() + t2.len());
    }
}
