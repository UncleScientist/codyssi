use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
    io::Error,
    ops::{Deref, DerefMut},
    str::FromStr,
};

pub fn run() -> Result<(), Error> {
    let sections = crate::read_sections(21, "Spiralling Stairs")?;

    let stairs = sections[0]
        .iter()
        .map(|stair| stair.parse::<Stair>().unwrap())
        .collect::<Vec<_>>();
    for (idx, stair) in stairs.iter().enumerate() {
        assert_eq!(idx, stair.id);
    }
    let last_step = stairs[0].last_step;

    let (_, magnitudes) = sections[1][0].split_once(" : ").unwrap();
    let magnitudes = magnitudes
        .split(", ")
        .map(|num| num.parse::<u128>().unwrap())
        .collect::<Vec<_>>();

    let mut ways = CountWays::default();
    println!(
        "  part 1 = {}",
        ways.count(
            (stairs[0].last_step - stairs[0].first_step) as u128,
            &magnitudes
        )
    );
    // 478223432388016434591719703

    let mut counter = CountGraphWays::default();
    let graph = Graph::generate_graph(&stairs, Location::new(0, last_step));
    let max = *magnitudes.iter().max().unwrap();
    let mags = magnitudes.into_iter().collect::<HashSet<_>>();

    let start = Location::new(0, 0);
    println!("  part 2 = {}", counter.count(&start, &graph, &mags, max));
    // 13107145891947202031341696509465277

    // S1_0-S1_1-S2_3-S1_3-S1_4-S1_5-S1_6
    let answer = path_for_target(100000000000000000000000000000, &graph, &counter, &mags);
    if answer == "S1_0-S1_1-S1_2-S1_3-S1_4-S1_5-S1_6-S1_7-S1_8-S1_9-S1_10-S1_11-S1_12-S1_13-S1_14-S1_16-S1_17-S1_18-S1_19-S1_24-S1_26-S1_29-S95_31-S95_32-S87_33-S87_34-S87_35-S87_36-S87_37-S87_38-S87_39-S87_40-S87_41-S87_49-S87_51-S87_53-S87_55-S87_56-S11_58-S11_59-S11_60-S1_61-S1_64-S1_65-S2_65-S2_68-S2_71-S2_72-S2_73-S2_74-S2_75-S2_76-S19_77-S19_78-S31_79-S31_80-S31_81-S93_81-S93_83-S93_85-S93_87-S17_89-S17_90-S7_90-S1_97".to_string() 
    || answer == "S1_0-S1_1-S1_2-S1_3-S1_4-S1_5-S1_6-S1_7-S1_8-S1_9-S1_10-S1_11-S1_12-S1_13-S1_14-S1_16-S1_17-S1_18-S1_19-S1_24-S1_26-S1_29-S95_31-S95_32-S87_33-S87_34-S87_35-S87_36-S87_37-S87_38-S87_39-S87_40-S87_41-S87_49-S87_51-S87_53-S87_55-S87_56-S11_58-S11_59-S11_60-S1_61-S1_64-S1_65-S2_65-S2_68-S2_71-S2_72-S2_73-S2_74-S2_75-S2_76-S19_77-S19_78-S31_79-S31_80-S31_81-S93_81-S93_83-S93_85-S93_87-S17_89-S17_90-S7_90-S99_94-S99_95-S1_96-S1_97" {
        println!("bad answer");
    }
    println!("  part 3 = {answer}");

    Ok(())
}

fn path_for_target(
    mut target: u128,
    graph: &Graph,
    counter: &CountGraphWays,
    mags: &HashSet<u128>,
) -> String {
    let max = *mags.iter().max().unwrap();
    let mut current = Location::new(0, 0);
    let mut answer = Vec::from([current.clone()]);
    'done: while let Some(neighbors) = graph.paths_from(&current, max, &mags) {
        let mut neighbors = neighbors.iter().collect::<Vec<_>>();
        let last = neighbors.len() == 1;
        // if neighbors.len() == 1 {
        // answer.push(*neighbors[0]);
        // break;
        // }
        neighbors.sort();
        println!("{target:30} | {current} -> {neighbors:?}");
        for neighbor in neighbors {
            let Some(ways) = counter.cache.get(&neighbor) else {
                if last {
                    answer.push(*neighbor);
                    break 'done;
                }
                continue;
            };
            if *ways < target {
                println!("{:30} | ways = {ways}, target = {target} @ {neighbor}", "");
                target -= *ways;
                println!("{:30} |     new target = {target}", "");
            } else {
                current = *neighbor;
                answer.push(*neighbor);
                println!("{:30} | {ways} ways, added {current}", "");
                break;
            }
        }
    }
    println!("final target = {target}");
    answer
        .into_iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>()
        .join("-")
}

type GraphType = HashMap<Location, HashSet<Location>>;
struct Graph {
    last: Location,
    graph: GraphType,
}

impl Graph {
    fn new(last: Location) -> Self {
        Self {
            last,
            graph: HashMap::new(),
        }
    }

    fn generate_graph(stairs: &[Stair], last: Location) -> Graph {
        let mut graph = Graph::new(last);
        for stair in stairs {
            if let StairLoc::Stair(staircase) = stair.start {
                graph
                    .entry(Location::new(staircase, stair.first_step))
                    .or_default()
                    .insert(Location::new(stair.id, stair.first_step));
            }
            if let StairLoc::Stair(staircase) = stair.end {
                graph
                    .entry(Location::new(stair.id, stair.last_step))
                    .or_default()
                    .insert(Location::new(staircase, stair.last_step));
            }
            let mut current = Location::new(stair.id, stair.first_step);
            while current.step != stair.last_step {
                graph.entry(current).or_default().insert(current.next());
                current = current.next();
            }
        }
        graph
    }

    fn paths_from(
        &self,
        loc: &Location,
        max: u128,
        allowed: &HashSet<u128>,
    ) -> Option<HashSet<Location>> {
        let mut queue = Vec::from(&[(*loc, 0)]);
        let mut visited = HashSet::new();
        let mut found = HashSet::new();

        while let Some((loc, dist)) = queue.pop() {
            if visited.insert((loc, dist)) {
                if let Some(set) = self.get(&loc) {
                    let next_dist = dist + 1;
                    for next in set {
                        assert!(next_dist <= max);
                        if allowed.contains(&next_dist) {
                            found.insert(*next);
                        }
                        if next_dist < max {
                            queue.push((*next, next_dist));
                        }
                    }
                }
            }
        }
        if found.is_empty() {
            None
        } else {
            Some(found)
        }
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl Deref for Graph {
    type Target = GraphType;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

#[derive(Debug, Default)]
struct CountWays {
    cache: HashMap<u128, u128>, // maps remaining steps to answer for that number of steps
}

impl CountWays {
    fn count(&mut self, remaining: u128, mags: &[u128]) -> u128 {
        if let Some(result) = self.cache.get(&remaining) {
            return *result;
        }
        let mut total = 0;
        for mag in mags {
            match (*mag).cmp(&remaining) {
                Ordering::Less => total += self.count(remaining - *mag, mags),
                Ordering::Equal => total += 1,
                Ordering::Greater => {}
            }
        }
        self.cache.insert(remaining, total);
        total
    }
}

#[derive(Debug, Default)]
struct CountGraphWays {
    cache: HashMap<Location, u128>,
}

impl CountGraphWays {
    fn count(&mut self, start: &Location, graph: &Graph, mags: &HashSet<u128>, max: u128) -> u128 {
        if let Some(result) = self.cache.get(start) {
            return *result;
        }
        let mut total = 0;
        if let Some(steps) = graph.paths_from(start, max, mags) {
            for step in steps {
                if step == graph.last {
                    total += 1;
                } else {
                    total += self.count(&step, graph, mags, max);
                }
            }
        }
        self.cache.insert(*start, total);
        total
    }
}

#[derive(Debug)]
struct Stair {
    id: usize,
    first_step: usize,
    last_step: usize,
    start: StairLoc,
    end: StairLoc,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum StairLoc {
    Start,
    End,
    Stair(usize),
}

impl FromStr for Stair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" : ").collect::<Vec<_>>();
        let id = parts[0][1..].parse::<usize>().unwrap();
        let (from, to) = parts[1].split_once(" -> ").unwrap();
        let start_end = parts[2].split(' ').collect::<Vec<_>>();
        Ok(Stair {
            id: id - 1,
            first_step: from.parse().unwrap(),
            last_step: to.parse().unwrap(),
            start: start_end[1].parse::<StairLoc>().unwrap(),
            end: start_end[3].parse::<StairLoc>().unwrap(),
        })
    }
}

impl FromStr for StairLoc {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "START" => Self::Start,
            "END" => Self::End,
            _ => Self::Stair(s[1..].parse::<usize>().unwrap() - 1),
        })
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone)]
struct Location {
    staircase: usize,
    step: usize,
}

impl Location {
    fn new(staircase: usize, step: usize) -> Self {
        Self { staircase, step }
    }

    fn next(&self) -> Self {
        Self::new(self.staircase, self.step + 1)
    }
}

impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{}_{}", self.staircase + 1, self.step)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_TINY: [&str; 2] = [
        "S1 : 0 -> 6 : FROM START TO END",
        "S2 : 2 -> 3 : FROM S1 TO S1",
    ];

    const TEST_MID: [&str; 3] = [
        "S1 : 0 -> 6 : FROM START TO END",
        "S2 : 2 -> 4 : FROM S1 TO S1",
        "S3 : 3 -> 5 : FROM S2 TO S1",
    ];

    const TEST_INPUT: [&str; 10] = [
        "S1 : 0 -> 99 : FROM START TO END",
        "S2 : 8 -> 91 : FROM S1 TO S1",
        "S3 : 82 -> 91 : FROM S1 TO S1",
        "S4 : 90 -> 97 : FROM S2 TO S1",
        "S5 : 29 -> 74 : FROM S1 TO S1",
        "S6 : 87 -> 90 : FROM S3 TO S2",
        "S7 : 37 -> 71 : FROM S2 TO S1",
        "S8 : 88 -> 90 : FROM S6 TO S3",
        "S9 : 34 -> 37 : FROM S2 TO S5",
        "S10 : 13 -> 57 : FROM S1 TO S2",
    ];

    #[test]
    fn test_tiny_graph() {
        let stairs = TEST_TINY
            .iter()
            .map(|stair| stair.parse::<Stair>().unwrap())
            .collect::<Vec<_>>();
        let graph = Graph::generate_graph(&stairs, Location::new(0, 6));

        let mut ways = CountGraphWays::default();
        let start = Location::new(0, 0);
        let mags = HashSet::from([1, 3]);
        assert_eq!(17, ways.count(&start, &graph, &mags, 3));
    }

    #[test]
    fn test_134() {
        let stairs = TEST_TINY
            .iter()
            .map(|stair| stair.parse::<Stair>().unwrap())
            .collect::<Vec<_>>();
        let graph = Graph::generate_graph(&stairs, Location::new(0, 6));
        let allowed = HashSet::from([1, 3, 4]);
        let Some(paths) = graph.paths_from(&Location::new(0, 2), 4, &allowed) else {
            panic!("no paths");
        };
        // straight path
        assert!(paths.contains(&Location::new(0, 3)));
        assert!(paths.contains(&Location::new(0, 5)));
        assert!(paths.contains(&Location::new(0, 6)));

        // path through S2
        assert!(paths.contains(&Location::new(1, 2)));
        assert!(paths.contains(&Location::new(0, 4)));
    }

    #[test]
    fn test_rank_39() {
        let stairs = TEST_MID
            .iter()
            .map(|stair| stair.parse::<Stair>().unwrap())
            .collect::<Vec<_>>();
        let graph = Graph::generate_graph(&stairs, Location::new(0, 6));
        let allowed = HashSet::from([1, 2]);
        let mut counter = CountGraphWays::default();
        counter.count(&Location::new(0, 0), &graph, &allowed, 2);
        let answer = path_for_target(39, &graph, &counter, &allowed);
        assert_eq!("S1_0-S1_1-S1_2-S2_3-S3_4-S3_5-S1_6".to_string(), answer);
    }

    #[test]
    fn test_rank_73287437832782344() {
        let stairs = TEST_INPUT
            .iter()
            .map(|stair| stair.parse::<Stair>().unwrap())
            .collect::<Vec<_>>();
        let graph = Graph::generate_graph(&stairs, Location::new(0, 99));
        let allowed = HashSet::from([1, 3, 5, 6]);
        let mut counter = CountGraphWays::default();
        counter.count(&Location::new(0, 0), &graph, &allowed, 6);
        let answer = path_for_target(73287437832782344, &graph, &counter, &allowed);
        assert_eq!("S1_0-S1_1-S1_2-S1_3-S1_4-S1_5-S1_6-S1_7-S1_8-S1_9-S1_10-S1_11-S1_12-S1_13-S1_14-S1_15-S1_16-S1_17-S1_18-S1_19-S1_20-S1_21-S1_22-S1_23-S1_24-S1_25-S1_26-S1_29-S5_29-S5_30-S5_35-S5_36-S5_37-S5_38-S5_39-S5_40-S5_45-S5_46-S5_47-S5_48-S5_51-S5_52-S5_53-S5_54-S5_55-S5_58-S5_59-S5_62-S5_63-S5_64-S5_65-S5_66-S5_67-S5_70-S5_71-S5_72-S1_76-S1_79-S1_80-S3_84-S3_85-S3_86-S3_87-S3_90-S1_92-S1_93-S1_94-S1_95-S1_98-S1_99".to_string(), answer);
    }
}

// S9 :  34---35---36---37
//                      v
// S5 :                 37---38---39---40 ...

// S1 :    0----1----2----3----4----5----6----7----8----9---10---11---12 ...
//                                                 v
// S2 :                                            8----9---10---11---12 ...
//
// S1 :   90---91---92---93---94---95---96---97---98---99
//             ^
// S2 :   90---91

// S1 :    0----1----2----3----4----5----6
//                   v    ^
// S2 :              2----3
//
// ----------------
//
// S1 :    0----1----2----3----4----5----6
//                   v         ^    ^
// S2 :              2----3----4    ^
//                        v         ^
// S3 :                   3----4----5
