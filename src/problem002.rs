use std::io::Error;

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem002.txt")?;

    println!("Puzzle 2: Sensors and Circuits");

    let lines = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let sensors = lines.iter().map(|text| *text == "TRUE").collect::<Vec<_>>();

    println!(
        "  part 1 = {}",
        sensors
            .iter()
            .enumerate()
            .filter_map(|(idx, val)| if *val { Some(idx + 1) } else { None })
            .sum::<usize>()
    );

    let layer = |(idx, pair): (usize, &[bool])| {
        if idx % 2 == 0 {
            pair[0] && pair[1]
        } else {
            pair[0] || pair[1]
        }
    };

    println!(
        "  part 2 = {}",
        sensors
            .chunks(2)
            .enumerate()
            .map(layer)
            .filter(|val| *val)
            .count()
    );

    let mut cur = sensors.clone();
    let mut total = cur.iter().filter(|v| **v).count();
    while cur.len() > 1 {
        let next_layer = cur.chunks(2).enumerate().map(layer).collect::<Vec<_>>();
        total += next_layer.iter().filter(|v| **v).count();
        cur = next_layer;
    }
    println!("  part 3 = {total}");

    Ok(())
}
