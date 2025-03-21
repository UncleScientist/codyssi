use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem005.txt")?;

    println!("Puzzle 5: Compass Calibration");

    let mut lines = data.split('\n').collect::<Vec<_>>();
    lines.pop();
    let ops = lines.pop().unwrap();
    let nums = lines
        .iter()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    println!(
        "  part 1 = {}",
        nums.iter()
            .skip(1)
            .zip(ops.chars())
            .fold(nums[0], |total, (num, op)| {
                if op == '+' {
                    total + num
                } else if op == '-' {
                    total - num
                } else {
                    panic!("invalid op {op}");
                }
            })
    );

    println!(
        "  part 2 = {}",
        nums.iter()
            .skip(1)
            .zip(ops.chars().rev())
            .fold(nums[0], |total, (num, op)| {
                if op == '+' {
                    total + num
                } else if op == '-' {
                    total - num
                } else {
                    panic!("invalid op {op}");
                }
            })
    );

    let bignums = nums
        .chunks(2)
        .map(|nvec| nvec[0] * 10 + nvec[1])
        .collect::<Vec<_>>();

    println!(
        "  part 3 = {}",
        bignums
            .iter()
            .skip(1)
            .zip(ops.chars().rev())
            .fold(bignums[0], |total, (num, op)| {
                if op == '+' {
                    total + num
                } else if op == '-' {
                    total - num
                } else {
                    panic!("invalid op {op}");
                }
            })
    );
    Ok(())
}
