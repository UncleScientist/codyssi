use std::{io::Error, str::FromStr};

// 10 by 15 by 60 by 3

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(0, "Cataclysmic Escape")?;

    let rules = lines
        .iter()
        .map(|line| line.parse::<Rule>().unwrap())
        .collect::<Vec<_>>();
    println!("{rules:?}");

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
    velocity: (isize, isize, isize, isize),
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
        let velocity = (
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
            velocity,
        })
    }
}
