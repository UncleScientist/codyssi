use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
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

    let mut queue = BinaryHeap::from([(Reverse(0), "STT".to_string())]);
    let mut visited = HashSet::new();
    let mut lengths = HashMap::<String, usize>::new();
    while let Some((Reverse(cost), loc)) = queue.pop() {
        if visited.insert(loc.clone()) {
            if !paths.contains_key(&loc) {
                continue;
            }
            for neighbor in &paths[&loc] {
                let newcost = cost + 1; // part 1: just use 1 for the length
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
    let mut values = lengths.values().copied().collect::<Vec<_>>();
    values.sort();
    println!(
        "  part 1 = {}",
        values.iter().rev().take(3).product::<usize>()
    );

    Ok(())
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
