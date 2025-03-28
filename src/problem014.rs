use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    io::Error,
};

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(14, "Cyclops Chaos")?;

    let mut grid = Vec::<Vec<i64>>::new();

    for line in &lines {
        grid.push(line.split(' ').map(|num| num.parse().unwrap()).collect());
    }

    let gridsize = grid.len();

    let mut safest = i64::MAX;
    for row in &grid {
        let rowsum = row.iter().sum::<i64>();
        let colsum = (0..gridsize).map(|col| row[col]).sum::<i64>();
        safest = safest.min(rowsum.min(colsum));
    }
    println!("  part 1 = {safest}");

    println!("  part 2 = {}", search(&grid, (14, 14)).unwrap());

    println!(
        "  part 3 = {}",
        search(&grid, (gridsize - 1, gridsize - 1)).unwrap()
    );
    Ok(())
}

fn search(grid: &[Vec<i64>], dest: (usize, usize)) -> Option<i64> {
    let mut queue = BinaryHeap::from([(Reverse(grid[0][0]), (0usize, 0usize))]);
    let mut visited = HashSet::new();
    let mut costs = HashMap::<(usize, usize), i64>::new();
    while let Some((Reverse(cost), pos)) = queue.pop() {
        if pos == dest {
            return Some(cost);
        }
        if visited.insert(pos) {
            for dir in [(1, 0), (0, 1)] {
                let newpos = (pos.0 + dir.0, pos.1 + dir.1);
                if newpos.0 >= grid.len() || newpos.1 >= grid[0].len() {
                    continue;
                }

                let newcost = cost + grid[newpos.0][newpos.1];
                if !visited.contains(&newpos) {
                    let entry = costs.entry(pos).or_insert(i64::MAX);
                    if newcost < *entry {
                        *entry = newcost;
                    }
                }
                queue.push((Reverse(newcost), newpos));
            }
        }
    }

    None
}
