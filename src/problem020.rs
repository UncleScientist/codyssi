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

    // cube._print();
    // println!("{:?}", instructions[0]);
    cube.action(instructions[0]);
    // cube._print();
    for (twist, inst) in twists.iter().zip(instructions.iter().skip(1)) {
        // println!("{twist:?}");
        cube.twist(*twist);
        // println!("{inst:?}");
        cube.action(*inst);
        // cube._print();
    }
    let mut values = cube.absorption;
    values.sort();
    println!(
        "  part 1 = {}",
        values.iter().rev().take(2).product::<u128>()
    );

    println!("  part 2 = {}", cube.dominant_sums());
    // incorrect: 60449500975162539098880

    Ok(())
}

#[derive(Debug, Clone)]
struct CubeFace {
    face: Vec<Vec<u8>>,
}

impl CubeFace {
    fn new(size: usize) -> Self {
        Self {
            face: vec![vec![1; size]; size],
        }
    }

    fn _print(&self) {
        for row in &self.face {
            for cell in row {
                print!(" {cell:2}");
            }
            println!();
        }
    }

    // 1 2 3                    7 4 1
    // 4 5 6 -> rotate right -> 8 5 2
    // 7 8 9                    9 6 3
    fn rotate_right(&mut self) {
        let size = self.face.len();
        let mut result = vec![vec![0; size]; size];
        for i in 0..size {
            for j in 0..size {
                result[j][size - 1 - i] = self.face[i][j];
            }
        }
        self.face = result;
    }

    // 1 2 3                   3 6 9
    // 4 5 6 -> rotate left -> 2 5 8
    // 7 8 9                   1 4 7
    fn rotate_left(&mut self) {
        let size = self.face.len();
        let mut result = vec![vec![0; size]; size];
        for i in 0..size {
            for j in 0..size {
                result[size - 1 - j][i] = self.face[i][j];
            }
        }
        self.face = result;
    }

    fn add_all(&mut self, amt: u8) {
        for row in &mut self.face {
            for cell in row {
                *cell = (*cell + amt - 1) % 100 + 1;
            }
        }
    }

    fn add_row(&mut self, row: usize, amt: u8) {
        for cell in &mut self.face[row] {
            *cell = (*cell + amt - 1) % 100 + 1;
        }
    }

    fn add_col(&mut self, col: usize, amt: u8) {
        for row in &mut self.face {
            row[col] = (row[col] + amt - 1) % 100 + 1;
        }
    }

    fn dominant_sum(&self) -> u128 {
        let rowmax = self
            .face
            .iter()
            .map(|row| row.iter().fold(0u128, |sum, entry| sum + (*entry as u128)))
            .max()
            .unwrap();
        let mut colmax = 0;
        for col in 0..self.face.len() {
            colmax = colmax.max(self.face.iter().map(|row| row[col] as u128).sum::<u128>())
        }
        rowmax.max(colmax)
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
    absorption: [u128; 6],
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

    fn _print(&self) {
        println!("forward:");
        self.faces[self.forward]._print();
        println!("back:");
        self.faces[self.back]._print();
        println!("left:");
        self.faces[self.left]._print();
        println!("right:");
        self.faces[self.right]._print();
        println!("top:");
        self.faces[self.top]._print();
        println!("bottom:");
        self.faces[self.bottom]._print();
    }

    fn action(&mut self, inst: Instruction) {
        let size = self.faces[0].face.len() as u128;
        match inst {
            Instruction::Face(amt) => {
                self.faces[self.forward].add_all(amt);
                self.absorption[self.forward] += (amt as u128) * size * size;
            }
            Instruction::Row(row, amt) => {
                self.faces[self.forward].add_row(row, amt);
                self.absorption[self.forward] += (amt as u128) * size;
            }
            Instruction::Col(col, amt) => {
                self.faces[self.forward].add_col(col, amt);
                self.absorption[self.forward] += (amt as u128) * size;
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
                self.faces[self.top].rotate_left();
                self.faces[self.bottom].rotate_right();
            }
            Twist::Right => {
                let tmp = self.right;
                self.right = self.back;
                self.back = self.left;
                self.left = self.forward;
                self.forward = tmp;
                self.faces[self.top].rotate_right();
                self.faces[self.bottom].rotate_left();
            }
            Twist::Up => {
                let tmp = self.top;
                self.top = self.back;
                self.back = self.bottom;
                self.bottom = self.forward;
                self.forward = tmp;
                self.faces[self.left].rotate_right();
                self.faces[self.right].rotate_left();
                self.faces[self.back].rotate_right();
                self.faces[self.back].rotate_right();
                self.faces[self.top].rotate_right();
                self.faces[self.top].rotate_right();
            }
            Twist::Down => {
                let tmp = self.bottom;
                self.bottom = self.back;
                self.back = self.top;
                self.top = self.forward;
                self.forward = tmp;
                self.faces[self.left].rotate_left();
                self.faces[self.right].rotate_right();
                self.faces[self.back].rotate_right();
                self.faces[self.back].rotate_right();
                self.faces[self.bottom].rotate_right();
                self.faces[self.bottom].rotate_right();
            }
        }
    }

    fn dominant_sums(&self) -> u128 {
        self.faces.iter().map(|face| face.dominant_sum()).product()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate_right() {
        let v1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let v2 = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        let mut face = CubeFace::new(3);
        face.face = v1;
        face.rotate_right();
        assert_eq!(v2, face.face);
    }

    #[test]
    fn test_rotate_left() {
        let v1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let v2 = vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]];
        let mut face = CubeFace::new(3);
        face.face = v1;
        face.rotate_left();
        assert_eq!(v2, face.face);
    }

    #[test]
    fn test_cube_twist_up() {
        let mut cube = Cube::new(3);
        cube.faces[cube.forward].face[0][0] = 10;
        cube.faces[cube.right].face[0][0] = 11;
        cube.faces[cube.back].face[0][0] = 12;
        cube.faces[cube.left].face[0][0] = 13;
        cube.faces[cube.top].face[0][0] = 14;
        cube.faces[cube.bottom].face[0][0] = 15;
        cube.twist(Twist::Up); // make the "up" face now the "forward" face
        assert_eq!(cube.faces[cube.bottom].face[0][0], 10);
        assert_eq!(cube.faces[cube.right].face[2][0], 11);
        assert_eq!(cube.faces[cube.top].face[2][2], 12);
        assert_eq!(cube.faces[cube.left].face[0][2], 13);
        assert_eq!(cube.faces[cube.forward].face[0][0], 14);
        assert_eq!(cube.faces[cube.back].face[2][2], 15);
    }

    #[test]
    fn test_cube_twist_right() {
        let mut cube = Cube::new(3);
        cube.faces[cube.forward].face[0][0] = 10;
        cube.faces[cube.right].face[0][0] = 11;
        cube.faces[cube.back].face[0][0] = 12;
        cube.faces[cube.left].face[0][0] = 13;
        cube.faces[cube.top].face[0][0] = 14;
        cube.faces[cube.bottom].face[0][0] = 15;
        cube.twist(Twist::Right); // make the "right" face now the "forward" face
        assert_eq!(cube.faces[cube.left].face[0][0], 10);
        assert_eq!(cube.faces[cube.forward].face[0][0], 11);
        assert_eq!(cube.faces[cube.right].face[0][0], 12);
        assert_eq!(cube.faces[cube.back].face[0][0], 13);
        assert_eq!(cube.faces[cube.top].face[0][2], 14);
        assert_eq!(cube.faces[cube.bottom].face[2][0], 15);
    }
}
