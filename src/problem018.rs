use std::{io::Error, str::FromStr};

pub fn run() -> Result<(), Error> {
    let lines = crate::read_and_split(18, "Crucial Crafting")?;

    let mut items = lines
        .iter()
        .map(|line| line.parse::<Item>().unwrap())
        .collect::<Vec<_>>();

    items.sort();

    println!(
        "  part 1 = {}",
        items
            .iter()
            .rev()
            .take(5)
            .map(|item| item.materials)
            .sum::<usize>()
    );
    println!("  part 2 = {}", knapsack_with_cost(&items, 30));
    println!("  part 3 = {}", knapsack_with_cost(&items, 300));
    Ok(())
}

fn knapsack_with_cost(items: &[Item], available_cost: usize) -> usize {
    // tuple: (quality, materials)
    // item    0      1       2       3         25    ... 30
    //   0     (0,0)  (0,0)   (0,0)   (0,0) ..  (0,0) ... (0,0)
    //   1     (0,0)  (0,0)   (0,0)   (0,0) .. (25,7)
    //   2     (0,0)
    //   3     (0,0)
    //   4     (0,0)
    //   ..
    //   nth   ........................................ (best cost, best materials)

    let mut knapsack = vec![vec![(0usize, 0usize); available_cost + 1]; items.len() + 1];
    for idx in 1..=items.len() {
        for cost in 1..=available_cost {
            let value_without_item = knapsack[idx - 1][cost];
            let cur_item_cost = items[idx - 1].cost;
            if cost >= cur_item_cost {
                let remaining_cost = cost - cur_item_cost;
                let prev_item = knapsack[idx - 1][remaining_cost];
                let new_value = (
                    items[idx - 1].quality + prev_item.0,
                    items[idx - 1].materials + prev_item.1,
                );
                if new_value.0 > value_without_item.0
                    || (new_value.0 == value_without_item.0 && new_value.1 < value_without_item.1)
                {
                    knapsack[idx][cost] = new_value;
                } else {
                    knapsack[idx][cost] = value_without_item;
                }
            } else {
                knapsack[idx][cost] = value_without_item;
            }
        }
    }
    let best = knapsack[items.len()][available_cost];
    best.0 * best.1
}

#[derive(Debug, PartialEq, Eq)]
struct Item {
    _id: usize,
    _code: String,
    quality: usize,
    cost: usize,
    materials: usize,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.quality == other.quality {
            self.cost.cmp(&other.cost)
        } else {
            self.quality.cmp(&other.quality)
        }
    }
}

impl FromStr for Item {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (left, right) = line.split_once(" | ").unwrap();

        let words = left.split(' ').collect::<Vec<_>>();
        let _id = words[0].parse().unwrap();
        let _code = words[1].to_string();

        let attrib = right.split(", ").collect::<Vec<_>>();
        let quality = attrib[0].split_once(" : ").unwrap().1.parse().unwrap();
        let cost = attrib[1].split_once(" : ").unwrap().1.parse().unwrap();
        let materials = attrib[2].split_once(" : ").unwrap().1.parse().unwrap();
        Ok(Self {
            _id,
            _code,
            quality,
            cost,
            materials,
        })
    }
}
