use std::io::Error;

pub fn run() -> Result<(), Error> {
    let mut lines = crate::read_and_split(5, "Compass Calibration")?;

    let ops = lines.pop().unwrap();
    let nums = lines
        .iter()
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let bignums = nums
        .chunks(2)
        .map(|nvec| nvec[0] * 10 + nvec[1])
        .collect::<Vec<_>>();

    println!("  part 1 = {}", calc(&nums, ops.chars()));
    println!("  part 2 = {}", calc(&nums, ops.chars().rev()));
    println!("  part 3 = {}", calc(&bignums, ops.chars().rev()));

    Ok(())
}

fn calc<I: Iterator<Item = char>>(nums: &[i64], iter: I) -> i64 {
    nums.iter()
        .skip(1)
        .zip(iter)
        .fold(nums[0], |total, (num, op)| {
            if op == '+' {
                total + num
            } else if op == '-' {
                total - num
            } else {
                panic!("invalid op {op}");
            }
        })
}
