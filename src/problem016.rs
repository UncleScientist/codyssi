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

    let instructions = sections[1]
        .split('\n')
        .map(|line| line.parse::<Oper>().unwrap())
        .collect::<Vec<_>>();

    let actions = sections[2]
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Action>().unwrap())
        .collect::<Vec<_>>();

    let charybdis = Charybdis {
        instructions,
        actions,
    };

    let mut part1 = values.clone();
    charybdis.execute_all(&mut part1);
    println!("  part 1 = {}", rowcolmax(&part1));

    let mut part2 = values.clone();
    let mut p2inst = VecDeque::from(charybdis.instructions.clone());
    charybdis.do_actions(&mut part2, &mut p2inst);
    println!("  part 2 = {}", rowcolmax(&part2));

    let mut part3 = values.clone();
    let mut p3inst = VecDeque::from(charybdis.instructions.clone());
    while !p3inst.is_empty() {
        charybdis.do_actions(&mut part3, &mut p3inst);
    }
    println!("  part 3 = {}", rowcolmax(&part3));

    Ok(())
}

#[derive(Debug, Clone)]
struct Charybdis {
    instructions: Vec<Oper>,
    actions: Vec<Action>,
}

impl Charybdis {
    fn do_actions(&self, values: &mut [Vec<i64>], instructions: &mut VecDeque<Oper>) {
        let mut cur_inst = None;
        for action in &self.actions {
            if cur_inst.is_none() && instructions.is_empty() {
                return;
            }
            match action {
                Action::Take => {
                    cur_inst = instructions.pop_front();
                }
                Action::Cycle => {
                    instructions.push_back(cur_inst.unwrap());
                }
                Action::Act => {
                    let Some(inst) = cur_inst else {
                        panic!("Action on non-instruction");
                    };
                    self.execute(values, &inst);
                }
            }
        }
    }

    fn execute(&self, values: &mut [Vec<i64>], inst: &Oper) {
        match inst {
            Oper::ShiftRow(r, amt) => {
                values[*r].rotate_right(*amt);
            }
            Oper::ShiftCol(c, amt) => {
                for _ in 0..*amt {
                    let mut len = values.len();
                    let last = values[len - 1][*c];
                    len -= 1;
                    while len > 0 {
                        values[len][*c] = values[len - 1][*c];
                        len -= 1;
                    }
                    values[0][*c] = last;
                }
            }
            Oper::Add(range, amt) => match range {
                Range::Row(r) => {
                    for item in &mut values[*r] {
                        *item = (*item + *amt) % 1073741824;
                    }
                }
                Range::Col(c) => {
                    for row in values {
                        row[*c] = (row[*c] + *amt) % 1073741824;
                    }
                }
                Range::All => {
                    for row in values {
                        for item in row {
                            *item = (*item + *amt) % 1073741824;
                        }
                    }
                }
            },
            Oper::Sub(range, amt) => match range {
                Range::Row(r) => {
                    for item in &mut values[*r] {
                        *item = (*item - *amt) % 1073741824;
                    }
                }
                Range::Col(c) => {
                    for row in values {
                        row[*c] = (row[*c] - *amt) % 1073741824;
                    }
                }
                Range::All => {
                    for row in values {
                        for item in row {
                            *item = (*item - *amt) % 1073741824;
                        }
                    }
                }
            },
            Oper::Multiply(range, amt) => match range {
                Range::Row(r) => {
                    for item in &mut values[*r] {
                        *item = (*item * *amt) % 1073741824;
                    }
                }
                Range::Col(c) => {
                    for row in values {
                        row[*c] = (row[*c] * *amt) % 1073741824;
                    }
                }
                Range::All => {
                    for row in values {
                        for item in row {
                            *item = (*item * *amt) % 1073741824;
                        }
                    }
                }
            },
        }
    }

    fn execute_all(&self, values: &mut [Vec<i64>]) {
        for inst in &self.instructions {
            self.execute(values, inst);
        }
    }
}

fn rowcolmax(values: &[Vec<i64>]) -> i64 {
    let mut rowmax = 0;
    for row in values {
        rowmax = rowmax.max(row.iter().sum::<i64>());
    }

    let mut colmax = 0;
    for col in 0..values[0].len() {
        colmax = colmax.max(values.iter().map(|row| row[col]).sum::<i64>());
    }
    rowmax.max(colmax)
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

#[derive(Debug, Clone)]
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
