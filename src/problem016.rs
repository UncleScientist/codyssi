use std::{io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem016.txt")?;
    // let data = std::fs::read_to_string("test.txt")?;

    println!("Puzzle 16: Challenging the Whirlpool");

    let sections = data
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut values = sections[0]
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

    for inst in &instructions {
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
                    for row in &mut values {
                        row[*c] = (row[*c] + *amt) % 1073741824;
                    }
                }
                Range::All => {
                    for row in &mut values {
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
                    for row in &mut values {
                        row[*c] = (row[*c] - *amt) % 1073741824;
                    }
                }
                Range::All => {
                    for row in &mut values {
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
                    for row in &mut values {
                        row[*c] = (row[*c] * *amt) % 1073741824;
                    }
                }
                Range::All => {
                    for row in &mut values {
                        for item in row {
                            *item = (*item * *amt) % 1073741824;
                        }
                    }
                }
            },
        }
    }

    let mut rowmax = 0;
    for row in &values {
        rowmax = rowmax.max(row.iter().sum::<i64>());
    }

    let mut colmax = 0;
    for col in 0..values[0].len() {
        colmax = colmax.max(values.iter().map(|row| row[col]).sum::<i64>());
    }
    println!("  part 1 = {}", rowmax.max(colmax));

    Ok(())
}

#[derive(Debug)]
enum Range {
    Row(usize),
    Col(usize),
    All,
}

#[derive(Debug)]
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
