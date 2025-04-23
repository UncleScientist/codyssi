use std::{io::Error, str::FromStr};

// real input, and test input 2 = (10, 15, 60)
// test input 1 = (3, 3, 5)

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(22, "Cataclysmic Escape")?;

    let rules = lines
        .iter()
        .map(|line| line.parse::<Rule>().unwrap())
        .collect::<Vec<_>>();
    println!(
        "  part 1 = {}",
        rules
            .iter()
            .map(|rule| rule.count_debris((10, 15, 60)))
            .sum::<usize>()
    );

    Ok(())
}

#[derive(Debug)]
struct Rule {
    x: isize,
    y: isize,
    z: isize,
    a: isize,
    base: isize,
    remainder: isize,
    _velocity: (isize, isize, isize, isize),
}

impl Rule {
    fn count_debris(&self, (xsize, ysize, zsize): (isize, isize, isize)) -> usize {
        let mut count = 0;
        for x in 0..xsize {
            for y in 0..ysize {
                for z in 0..zsize {
                    for a in [-1, 0, 1] {
                        let sum = x * self.x + y * self.y + z * self.z + a * self.a;
                        let rem = sum.rem_euclid(self.base);
                        count += (rem == self.remainder) as usize;
                    }
                }
            }
        }
        count
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn get_coeff(word: &str) -> isize {
            word[0..word.len() - 1].parse().unwrap()
        }

        let words = line.split(' ').collect::<Vec<_>>();
        let coeff = words[2].split('+').collect::<Vec<_>>();
        let x = get_coeff(coeff[0]);
        let y = get_coeff(coeff[1]);
        let z = get_coeff(coeff[2]);
        let a = get_coeff(coeff[3]);
        let base = words[4].parse().unwrap();
        let remainder = words[7].parse().unwrap();

        let (_, velocities) = line.split_once('(').unwrap();
        let velocities = &velocities[0..velocities.len() - 1];
        let velocities = velocities.split(", ").collect::<Vec<_>>();
        let _velocity = (
            velocities[0].parse().unwrap(),
            velocities[1].parse().unwrap(),
            velocities[2].parse().unwrap(),
            velocities[3].parse().unwrap(),
        );
        Ok(Self {
            x,
            y,
            z,
            a,
            base,
            remainder,
            _velocity,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rule_1() {
        let rule = "RULE 1: 8x+2y+3z+5a DIVIDE 9 HAS REMAINDER 4 | DEBRIS VELOCITY (0, -1, 0, 1)"
            .parse::<Rule>()
            .unwrap();
        assert_eq!(14, rule.count_debris((3, 3, 5)));
    }

    #[test]
    fn test_rule_2() {
        let rule = "RULE 2: 4x+7y+10z+9a DIVIDE 5 HAS REMAINDER 4 | DEBRIS VELOCITY (0, 1, 0, 1)"
            .parse::<Rule>()
            .unwrap();
        assert_eq!(30, rule.count_debris((3, 3, 5)));
    }

    #[test]
    fn test_rule_3() {
        let rule = "RULE 3: 6x+3y+7z+3a DIVIDE 4 HAS REMAINDER 1 | DEBRIS VELOCITY (-1, 0, 1, -1)"
            .parse::<Rule>()
            .unwrap();
        assert_eq!(34, rule.count_debris((3, 3, 5)));
    }

    #[test]
    fn test_rule_4() {
        let rule =
            "RULE 4: 3x+11y+11z+3a DIVIDE 2 HAS REMAINDER 1 | DEBRIS VELOCITY (-1, 1, 0, -1)"
                .parse::<Rule>()
                .unwrap();
        assert_eq!(68, rule.count_debris((3, 3, 5)));
    }
}
