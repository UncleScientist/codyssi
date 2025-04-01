use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    io::Error,
    str::FromStr,
};

type SearchSpace = HashMap<String, Vec<(String, usize)>>;

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(17, "Laestrygonian Guards")?;

    let edges = lines
        .iter()
        .map(|line| line.parse::<Edge>().unwrap())
        .collect::<Vec<_>>();

    let mut paths = SearchSpace::new();
    for edge in &edges {
        paths
            .entry(edge.start.clone())
            .or_default()
            .push((edge.end.clone(), edge.length));
    }

    let mut values = get_all_lengths(&paths, false);
    values.sort();
    println!(
        "  part 1 = {}",
        values.iter().rev().take(3).product::<usize>()
    );

    let mut values = get_all_lengths(&paths, true);
    values.sort();
    println!(
        "  part 2 = {}",
        values.iter().rev().take(3).product::<usize>()
    );

    let keys = paths.keys().collect::<Vec<_>>();
    let mut cycle = 0;
    for key in keys {
        let mut queue = VecDeque::from([(0, key.clone())]);
        let mut visited = HashSet::new();
        while let Some((cost, pos)) = queue.pop_front() {
            if !visited.is_empty() && pos == *key {
                cycle = cycle.max(cost);
                break;
            }
            if visited.insert(pos.clone()) {
                let Some(neighbors) = paths.get(&pos) else {
                    continue;
                };
                for neighbor in neighbors {
                    queue.push_back((cost + neighbor.1, neighbor.0.clone()));
                }
            }
        }
    }
    println!("  part 3 = {cycle}");

    Ok(())
}

fn get_all_lengths(paths: &SearchSpace, use_cost: bool) -> Vec<usize> {
    let mut queue = BinaryHeap::from([(Reverse(0), "STT".to_string())]);
    let mut visited = HashSet::new();
    let mut lengths = HashMap::<String, usize>::new();
    while let Some((Reverse(cost), loc)) = queue.pop() {
        if visited.insert(loc.clone()) {
            if !paths.contains_key(&loc) {
                continue;
            }
            for neighbor in &paths[&loc] {
                let newcost = cost + if use_cost { neighbor.1 } else { 1 };
                if !visited.contains(&neighbor.0) {
                    let entry = lengths.entry(loc.clone()).or_insert(usize::MAX);
                    if newcost < *entry {
                        *entry = newcost;
                    }
                }
                queue.push((Reverse(newcost), neighbor.0.clone()));
            }
        }
    }
    lengths.values().copied().collect()
}

#[derive(Debug)]
struct Edge {
    start: String,
    end: String,
    length: usize,
}

impl FromStr for Edge {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (path, length) = line.split_once(" | ").unwrap();
        let length = length.parse().unwrap();
        let (from, to) = path.split_once(" -> ").unwrap();
        Ok(Self {
            start: from.to_string(),
            end: to.to_string(),
            length,
        })
    }
}
