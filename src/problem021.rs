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

    Ok(())
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

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
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
    fn test_generate_moves() {
        let _stairs = TEST_INPUT
            .iter()
            .map(|stair| stair.parse::<Stair>().unwrap())
            .collect::<Vec<_>>();

        /*let moves = get_moves(&stairs, &[1, 3, 5, 6], &Location::new(0, 6));
        assert!(moves.contains_key(&Location::new(0, 7)));
        assert!(moves.contains_key(&Location::new(1, 11)));

        let moves = get_moves(&stairs, &[1, 3, 5, 6], &Location::new(8, 36));
        dbg!(&moves);
        assert!(false);
        */
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
