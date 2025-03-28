use std::{collections::VecDeque, io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem016.txt")?;
    // let data = std::fs::read_to_string("test.txt")?;

    println!("Puzzle 16: Challenging the Whirlpool");

    let sections = data
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let values = sections[0]
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let grid = Grid { values };

    let instructions = sections[1]
        .split('\n')
        .map(|line| line.parse::<Oper>().unwrap())
        .collect::<Vec<_>>();

    let actions = sections[2]
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Action>().unwrap())
        .collect::<Vec<_>>();

    let mut part1 = grid.clone();
    for inst in &instructions {
        part1.execute(inst);
    }
    println!("  part 1 = {}", part1.rowcolmax());

    let mut part2 = grid.clone();
    let mut p2inst = VecDeque::from(instructions);

    let mut cur_inst = None;
    for action in &actions {
        match action {
            Action::Take => {
                cur_inst = p2inst.pop_front();
            }
            Action::Cycle => {
                p2inst.push_back(cur_inst.unwrap());
            }
            Action::Act => {
                let Some(inst) = cur_inst else {
                    panic!("Action on non-instruction");
                };
                part2.execute(&inst);
            }
        }
    }
    println!("  part 2 = {}", part2.rowcolmax());

    Ok(())
}

#[derive(Debug, Clone)]
struct Grid {
    values: Vec<Vec<i64>>,
}

impl Grid {
    fn execute(&mut self, inst: &Oper) {
        match inst {
            Oper::ShiftRow(r, amt) => {
                self.values[*r].rotate_right(*amt);
            }
            Oper::ShiftCol(c, amt) => {
                for _ in 0..*amt {
                    let mut len = self.values.len();
                    let last = self.values[len - 1][*c];
                    len -= 1;
                    while len > 0 {
                        self.values[len][*c] = self.values[len - 1][*c];
                        len -= 1;
                    }
                    self.values[0][*c] = last;
                }
            }
            Oper::Add(range, amt) => match range {
                Range::Row(r) => {
                    for item in &mut self.values[*r] {
                        *item = (*item + *amt) % 1073741824;
                    }
                }
                Range::Col(c) => {
                    for row in &mut self.values {
                        row[*c] = (row[*c] + *amt) % 1073741824;
                    }
                }
                Range::All => {
                    for row in &mut self.values {
                        for item in row {
                            *item = (*item + *amt) % 1073741824;
                        }
                    }
                }
            },
            Oper::Sub(range, amt) => match range {
                Range::Row(r) => {
                    for item in &mut self.values[*r] {
                        *item = (*item - *amt) % 1073741824;
                    }
                }
                Range::Col(c) => {
                    for row in &mut self.values {
                        row[*c] = (row[*c] - *amt) % 1073741824;
                    }
                }
                Range::All => {
                    for row in &mut self.values {
                        for item in row {
                            *item = (*item - *amt) % 1073741824;
                        }
                    }
                }
            },
            Oper::Multiply(range, amt) => match range {
                Range::Row(r) => {
                    for item in &mut self.values[*r] {
                        *item = (*item * *amt) % 1073741824;
                    }
                }
                Range::Col(c) => {
                    for row in &mut self.values {
                        row[*c] = (row[*c] * *amt) % 1073741824;
                    }
                }
                Range::All => {
                    for row in &mut self.values {
                        for item in row {
                            *item = (*item * *amt) % 1073741824;
                        }
                    }
                }
            },
        }
    }

    fn rowcolmax(&self) -> i64 {
        let mut rowmax = 0;
        for row in &self.values {
            rowmax = rowmax.max(row.iter().sum::<i64>());
        }

        let mut colmax = 0;
        for col in 0..self.values[0].len() {
            colmax = colmax.max(self.values.iter().map(|row| row[col]).sum::<i64>());
        }
        rowmax.max(colmax)
    }
}

#[derive(Debug, Copy, Clone)]
enum Range {
    Row(usize),
    Col(usize),
    All,
}

#[derive(Debug, Copy, Clone)]
enum Oper {
    ShiftRow(usize, usize),
    ShiftCol(usize, usize),
    Add(Range, i64),
    Sub(Range, i64),
    Multiply(Range, i64),
}

impl FromStr for Oper {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn get_rowcol(val: &str) -> usize {
            val.parse::<usize>().unwrap() - 1
        }

        let words = line.split(' ').collect::<Vec<_>>();
        Ok(match words[0] {
            "SHIFT" => match words[1] {
                "ROW" => Self::ShiftRow(get_rowcol(words[2]), words[4].parse().unwrap()),
                "COL" => Self::ShiftCol(get_rowcol(words[2]), words[4].parse().unwrap()),
                _ => panic!("Invalid instruction {line}"),
            },
            "ADD" => match words[2] {
                "ROW" => Self::Add(Range::Row(get_rowcol(words[3])), words[1].parse().unwrap()),
                "COL" => Self::Add(Range::Col(get_rowcol(words[3])), words[1].parse().unwrap()),
                "ALL" => Self::Add(Range::All, words[1].parse().unwrap()),
                _ => panic!("Invalid instruction {line}"),
            },
            "SUB" => match words[2] {
                "ROW" => Self::Sub(Range::Row(get_rowcol(words[3])), words[1].parse().unwrap()),
                "COL" => Self::Sub(Range::Col(get_rowcol(words[3])), words[1].parse().unwrap()),
                "ALL" => Self::Sub(Range::All, words[1].parse().unwrap()),
                _ => panic!("Invalid instruction {line}"),
            },
            "MULTIPLY" => match words[2] {
                "ROW" => {
                    Self::Multiply(Range::Row(get_rowcol(words[3])), words[1].parse().unwrap())
                }
                "COL" => {
                    Self::Multiply(Range::Col(get_rowcol(words[3])), words[1].parse().unwrap())
                }
                "ALL" => Self::Multiply(Range::All, words[1].parse().unwrap()),
                _ => panic!("Invalid instruction {line}"),
            },
            _ => panic!("Invalid instruction {line}"),
        })
    }
}

#[derive(Debug)]
enum Action {
    Take,
    Cycle,
    Act,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "TAKE" => Self::Take,
            "CYCLE" => Self::Cycle,
            "ACT" => Self::Act,
            _ => panic!("Invalid action {s}"),
        })
    }
}
