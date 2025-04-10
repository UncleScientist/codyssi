use std::{collections::HashMap, io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let sections = crate::read_sections(0, "Spiralling Stairs")?;

    let stairs = sections[0]
        .iter()
        .map(|stair| stair.parse::<Stair>().unwrap())
        .collect::<Vec<_>>();
    for (idx, stair) in stairs.iter().enumerate() {
        assert_eq!(idx, stair.id);
    }

    let (_, magnitudes) = sections[1][0].split_once(" : ").unwrap();
    let magnitudes = magnitudes
        .split(", ")
        .map(|num| num.parse::<u128>().unwrap())
        .collect::<Vec<_>>();

    let mut ways = CountWays::default();
    println!(
        "  part 1 = {}",
        ways.count((stairs[0].to - stairs[0].from) as u128, &magnitudes)
    );
    Ok(())
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
            if *mag == remaining {
                total += 1;
            } else if *mag < remaining {
                total += self.count(remaining - *mag, mags);
            }
        }
        self.cache.insert(remaining, total);
        total
    }
}

#[derive(Debug)]
struct Stair {
    id: usize,
    from: usize,
    to: usize,
    start: StairLoc,
    end: StairLoc,
}

impl Stair {
    fn reaches(&self, start: usize, dist: usize) -> bool {
        start + dist <= self.to
    }
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
            from: from.parse().unwrap(),
            to: to.parse().unwrap(),
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

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    staircase: usize,
    step: usize,
}

impl Location {
    fn new(staircase: usize, step: usize) -> Self {
        Self { staircase, step }
    }
}

// TODO: delete me
fn get_moves(stairs: &[Stair], moves: &[usize], loc: &Location) -> HashMap<Location, u128> {
    let mut result = HashMap::new();
    for dist in moves {
        if stairs[loc.staircase].reaches(loc.step, *dist) {
            *result
                .entry(Location::new(loc.staircase, loc.step + dist))
                .or_default() += 1;
        }
        let my_stair = &stairs[loc.staircase];
        for stair in stairs {
            if stair.id == loc.staircase {
                continue;
            }
            if stair.start == StairLoc::Stair(loc.staircase) && stair.from < loc.step + dist - 1 {
                *result
                    .entry(Location::new(stair.id, loc.step + dist - 1))
                    .or_default() += 1;
            }
            if stair.id == 4 {
                println!(
                    "{stair:?}, loc={}, dist={dist}, my_end={my_stair:?}",
                    loc.step
                );
            }
            if my_stair.to <= loc.step + dist - 1 && StairLoc::Stair(stair.id) == my_stair.end {
                println!("{stair:?} -> {} + {dist}", loc.step);
                *result
                    .entry(Location::new(stair.id, loc.step + dist - 1))
                    .or_default() += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn test_generate_moves() {
        let stairs = TEST_INPUT
            .iter()
            .map(|stair| stair.parse::<Stair>().unwrap())
            .collect::<Vec<_>>();

        let moves = get_moves(&stairs, &[1, 3, 5, 6], &Location::new(0, 6));
        assert!(moves.contains_key(&Location::new(0, 7)));
        assert!(moves.contains_key(&Location::new(1, 11)));

        let moves = get_moves(&stairs, &[1, 3, 5, 6], &Location::new(8, 36));
        dbg!(&moves);
        assert!(false);
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
