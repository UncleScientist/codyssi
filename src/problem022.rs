use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Error,
    str::FromStr,
};

// real input, and test input 2 = (10, 15, 60)
// test input 1 = (3, 3, 5)

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(22, "Cataclysmic Escape")?;

    let rules = lines
        .iter()
        .map(|line| line.parse::<Rule>().unwrap())
        .collect::<Vec<_>>();

    let mut space = Space::new(rules, (10, 15, 60));
    // let mut space = Space::new(rules, (3, 3, 5));

    println!("  part 1 = {}", space.debris.len());

    // prime the pump
    space.count_hits((1, 1, 1), 0);

    //
    // Part 2
    //
    let mut queue = VecDeque::from([((0, 0, 0), 0_usize)]);
    let mut visited = HashSet::new();

    while let Some((pos, time)) = queue.pop_front() {
        if space.found_exit(pos) {
            println!("  part 2 = {}", time);
            break;
        }
        if visited.insert((pos, time)) {
            for delta in [
                (0, 0, 0),
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, 1),
                (0, 0, -1),
            ] {
                let newpos = (pos.0 + delta.0, pos.1 + delta.1, pos.2 + delta.2);
                if !space.in_range(newpos) {
                    continue;
                }
                let newtime = time + 1;
                if !space.collides(newpos, newtime) {
                    queue.push_back((newpos, newtime));
                }
            }
        }
    }

    //
    // Part 3
    //
    let mut queue = VecDeque::from([((0, 0, 0), 0_usize, 0_usize)]);
    let mut visited = HashSet::new();

    while let Some((pos, time, hits)) = queue.pop_front() {
        if space.found_exit(pos) {
            println!("  part 3 = {}", time);
            break;
        }
        if visited.insert((pos, time, hits)) {
            for delta in [
                (0, 0, 0),
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, 1),
                (0, 0, -1),
            ] {
                let newpos = (pos.0 + delta.0, pos.1 + delta.1, pos.2 + delta.2);
                if !space.in_range(newpos) {
                    continue;
                }
                let newtime = time + 1;
                let newhits = hits + space.count_hits(newpos, newtime);
                if newhits > 3 {
                    continue;
                }
                queue.push_back((newpos, newtime, newhits));
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Space {
    _rules: Vec<Rule>,
    debris: Vec<Debris>,
    extent: (isize, isize, isize),
    cache: Vec<HashMap<(isize, isize, isize), usize>>,
}

impl Space {
    fn new(rules: Vec<Rule>, extent: (isize, isize, isize)) -> Self {
        let debris = rules.iter().flat_map(|rule| rule.debris(&extent)).collect();
        Self {
            _rules: rules,
            debris,
            extent,
            cache: Vec::new(),
        }
    }

    fn found_exit(&self, pos: (isize, isize, isize)) -> bool {
        pos.0 == self.extent.0 - 1 && pos.1 == self.extent.1 - 1 && pos.2 == self.extent.2 - 1
    }

    fn collides(&mut self, pos: (isize, isize, isize), time: usize) -> bool {
        if pos == (0, 0, 0) {
            return false;
        }
        self.count_hits(pos, time) != 0
    }

    fn in_range(&self, pos: (isize, isize, isize)) -> bool {
        !(pos.0 < 0
            || pos.0 >= self.extent.0
            || pos.1 < 0
            || pos.1 >= self.extent.1
            || pos.2 < 0
            || pos.2 >= self.extent.2)
    }

    fn count_hits(&mut self, pos: (isize, isize, isize), time: usize) -> usize {
        if pos == (0, 0, 0) {
            return 0;
        }
        if time >= self.cache.len() {
            assert_eq!(time, self.cache.len());
            let mut map = HashMap::new();
            for dpos in self
                .debris
                .iter()
                .map(|debris| debris.at(time, &self.extent))
            {
                if dpos.3 != 0 {
                    continue;
                }
                *map.entry((dpos.0, dpos.1, dpos.2)).or_default() += 1;
            }
            self.cache.push(map);
        }

        *self.cache[time].entry(pos).or_default()
    }
}

#[derive(Debug)]
struct Rule {
    x: isize,
    y: isize,
    z: isize,
    a: isize,
    base: isize,
    remainder: isize,
    velocity: (isize, isize, isize, isize),
}

impl Rule {
    fn debris(&self, (xsize, ysize, zsize): &(isize, isize, isize)) -> Vec<Debris> {
        let mut result = Vec::new();
        for x in 0..*xsize {
            for y in 0..*ysize {
                for z in 0..*zsize {
                    for a in [-1, 0, 1] {
                        let sum = x * self.x + y * self.y + z * self.z + a * self.a;
                        let rem = sum.rem_euclid(self.base);
                        if rem == self.remainder {
                            result.push(Debris {
                                pos: (x, y, z, a),
                                vel: self.velocity,
                            });
                        }
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug)]
struct Debris {
    pos: (isize, isize, isize, isize),
    vel: (isize, isize, isize, isize),
}

impl Debris {
    fn at(&self, time: usize, extent: &(isize, isize, isize)) -> (isize, isize, isize, isize) {
        let time = time as isize;
        let x = (self.pos.0 + time * self.vel.0).rem_euclid(extent.0);
        let y = (self.pos.1 + time * self.vel.1).rem_euclid(extent.1);
        let z = (self.pos.2 + time * self.vel.2).rem_euclid(extent.2);
        let a = (self.pos.3 + time * self.vel.3 + 1).rem_euclid(3) - 1;
        (x, y, z, a)
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn get_coeff(word: &str) -> isize {
            word[0..word.len() - 1].parse().unwrap()
        }

        let words = line.split(' ').collect::<Vec<_>>();
        let coeff = words[2].split('+').collect::<Vec<_>>();
        let x = get_coeff(coeff[0]);
        let y = get_coeff(coeff[1]);
        let z = get_coeff(coeff[2]);
        let a = get_coeff(coeff[3]);
        let base = words[4].parse().unwrap();
        let remainder = words[7].parse().unwrap();

        let (_, velocities) = line.split_once('(').unwrap();
        let velocities = &velocities[0..velocities.len() - 1];
        let velocities = velocities.split(", ").collect::<Vec<_>>();
        let _velocity = (
            velocities[0].parse().unwrap(),
            velocities[1].parse().unwrap(),
            velocities[2].parse().unwrap(),
            velocities[3].parse().unwrap(),
        );
        Ok(Self {
            x,
            y,
            z,
            a,
            base,
            remainder,
            velocity: _velocity,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rule_1() {
        let rule = "RULE 1: 8x+2y+3z+5a DIVIDE 9 HAS REMAINDER 4 | DEBRIS VELOCITY (0, -1, 0, 1)"
            .parse::<Rule>()
            .unwrap();
        assert_eq!(14, rule.debris(&(3, 3, 5)).len());
    }

    #[test]
    fn test_rule_2() {
        let rule = "RULE 2: 4x+7y+10z+9a DIVIDE 5 HAS REMAINDER 4 | DEBRIS VELOCITY (0, 1, 0, 1)"
            .parse::<Rule>()
            .unwrap();
        assert_eq!(30, rule.debris(&(3, 3, 5)).len());
    }

    #[test]
    fn test_rule_3() {
        let rule = "RULE 3: 6x+3y+7z+3a DIVIDE 4 HAS REMAINDER 1 | DEBRIS VELOCITY (-1, 0, 1, -1)"
            .parse::<Rule>()
            .unwrap();
        assert_eq!(34, rule.debris(&(3, 3, 5)).len());
    }

    #[test]
    fn test_rule_4() {
        let rule =
            "RULE 4: 3x+11y+11z+3a DIVIDE 2 HAS REMAINDER 1 | DEBRIS VELOCITY (-1, 1, 0, -1)"
                .parse::<Rule>()
                .unwrap();
        assert_eq!(68, rule.debris(&(3, 3, 5)).len());
    }

    #[test]
    fn test_debris_movement() {
        let debris = Debris {
            pos: (3, 9, 1, -1),
            vel: (1, -1, 0, 1),
        };
        assert_eq!((4, 8, 1, 0), debris.at(1, &(10, 15, 60)));
        assert_eq!((5, 7, 1, 1), debris.at(2, &(10, 15, 60)));
        assert_eq!((6, 6, 1, -1), debris.at(3, &(10, 15, 60)));
        assert_eq!((0, 2, 1, 0), debris.at(7, &(10, 15, 60)));
    }
}
