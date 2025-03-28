use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Error,
    str::FromStr,
};

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(4, "Traversing the Country")?;
    let edges = lines
        .iter()
        .map(|edge| edge.parse::<Edge>().unwrap())
        .collect::<Vec<_>>();

    let mut locations = HashMap::<String, HashSet<String>>::new();
    for edge in &edges {
        locations
            .entry(edge.left.clone())
            .or_default()
            .insert(edge.right.clone());
        locations
            .entry(edge.right.clone())
            .or_default()
            .insert(edge.left.clone());
    }
    println!("  part 1 = {}", locations.len());

    let mut queue = VecDeque::new();
    queue.push_back(("STT".to_string(), 0));
    let mut visited = HashSet::new();
    while let Some((loc, time)) = queue.pop_front() {
        if time > 3 {
            continue;
        }
        if visited.insert(loc.clone()) {
            for neighbor in &locations[&loc] {
                queue.push_back((neighbor.clone(), time + 1));
            }
        }
    }
    println!("  part 2 = {}", visited.len());

    let mut queue = VecDeque::new();
    queue.push_back(("STT".to_string(), 0));
    let mut visited = HashSet::new();
    let mut total_time = 0;
    while let Some((loc, time)) = queue.pop_front() {
        if visited.insert(loc.clone()) {
            total_time += time;
            for neighbor in &locations[&loc] {
                queue.push_back((neighbor.clone(), time + 1));
            }
        }
    }
    println!("  part 3 = {total_time}");

    Ok(())
}

#[derive(Debug)]
struct Edge {
    left: String,
    right: String,
}

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" <-> ").unwrap();
        Ok(Self {
            left: left.into(),
            right: right.into(),
        })
    }
}
