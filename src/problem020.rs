use std::{io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let sections = crate::read_sections(20, "Leviathan Mindscape")?;

    let instructions = sections[0]
        .iter()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect::<Vec<_>>();

    let twists: Vec<Twist> = sections[1][0]
        .chars()
        .map(|ch| ch.into())
        .collect::<Vec<_>>();

    Ok(())
}

#[derive(Debug)]
struct CubeFace {
    face: Vec<Vec<u8>>,
}

#[derive(Debug)]
struct Cube {
    faces: [CubeFace; 6],
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    back: usize,
    forward: usize, // this is the one facing the user
    absorbption: [usize; 6],
}

#[derive(Debug)]
enum Instruction {
    Face(u8),
    Row(usize, u8),
    Col(usize, u8),
}

#[derive(Debug)]
enum Twist {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (which, value) = line.split_once(" - VALUE ").unwrap();
        let value = value.parse().unwrap();
        Ok(if let Some((rc, index)) = which.split_once(' ') {
            let index = index.parse::<usize>().unwrap() - 1;
            match rc {
                "ROW" => Self::Row(index, value),
                "COL" => Self::Col(index, value),
                _ => panic!("Invalid instruction: {line}"),
            }
        } else if which == "FACE" {
            Self::Face(value)
        } else {
            panic!("Invalid instruction: {line}");
        })
    }
}

impl From<char> for Twist {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            'D' => Self::Down,
            'U' => Self::Up,
            _ => panic!("Invalid twist: {value}"),
        }
    }
}
