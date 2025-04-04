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

    let mut cube = Cube::new(80);

    cube.action(instructions[0]);
    for (twist, inst) in twists.iter().zip(instructions.iter().skip(1)) {
        cube.twist(*twist);
        cube.action(*inst);
    }
    let mut values = cube.absorption;
    values.sort();
    println!(
        "  part 1 = {}",
        values.iter().rev().take(2).product::<usize>()
    );

    Ok(())
}

#[derive(Debug, Clone)]
struct CubeFace {
    face: Vec<Vec<u8>>,
}

impl CubeFace {
    fn new(size: usize) -> Self {
        Self {
            face: vec![vec![0; size]; size],
        }
    }
}

#[derive(Debug)]
struct Cube {
    faces: Vec<CubeFace>,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
    back: usize,
    forward: usize, // this is the one facing the user
    absorption: [usize; 6],
}

impl Cube {
    fn new(size: usize) -> Self {
        Self {
            faces: vec![CubeFace::new(size); 6],
            forward: 0,
            top: 1,
            back: 2,
            bottom: 3,
            left: 4,
            right: 5,
            absorption: [0, 0, 0, 0, 0, 0],
        }
    }

    fn action(&mut self, inst: Instruction) {
        let size = self.faces[0].face.len();
        match inst {
            Instruction::Face(amt) => self.absorption[self.forward] += (amt as usize) * size * size,
            Instruction::Row(_, amt) => {
                self.absorption[self.forward] += (amt as usize) * size;
            }
            Instruction::Col(_, amt) => {
                self.absorption[self.forward] += (amt as usize) * size;
            }
        }
    }

    fn twist(&mut self, twist: Twist) {
        match twist {
            Twist::Left => {
                let tmp = self.left;
                self.left = self.back;
                self.back = self.right;
                self.right = self.forward;
                self.forward = tmp;
            }
            Twist::Right => {
                let tmp = self.right;
                self.right = self.back;
                self.back = self.left;
                self.left = self.forward;
                self.forward = tmp;
            }
            Twist::Up => {
                let tmp = self.top;
                self.top = self.back;
                self.back = self.bottom;
                self.bottom = self.forward;
                self.forward = tmp;
            }
            Twist::Down => {
                let tmp = self.bottom;
                self.bottom = self.back;
                self.back = self.top;
                self.top = self.forward;
                self.forward = tmp;
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Face(u8),
    Row(usize, u8),
    Col(usize, u8),
}

#[derive(Debug, Copy, Clone)]
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
