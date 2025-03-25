use std::{collections::HashMap, io::Error};

pub fn run() -> Result<(), Error> {
    let data = std::fs::read_to_string("input/problem013.txt")?;
    // let data = std::fs::read_to_string("test.txt")?;

    println!("Puzzle 13: Windy Bargain");

    let sections = data
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut starting_balances = HashMap::<String, i64>::new();
    for line in sections[0].split('\n') {
        let words = line.split(' ').collect::<Vec<_>>();
        starting_balances.insert(words[0].into(), words[2].parse::<i64>().unwrap());
    }
    // println!("{starting_balances:?}");

    let mut part1 = starting_balances.clone();
    for line in sections[1].split('\n') {
        if line.is_empty() {
            continue;
        }
        let words = line.split(' ').collect::<Vec<_>>();
        let amount = words[5].parse::<i64>().unwrap();
        *part1.get_mut(words[1]).unwrap() -= amount;
        *part1.get_mut(words[3]).unwrap() += amount;
    }
    let mut balances = part1.values().copied().collect::<Vec<_>>();
    balances.sort();
    println!("  part 1 = {}", balances.iter().rev().take(3).sum::<i64>());

    let mut part2 = starting_balances.clone();
    for line in sections[1].split('\n') {
        if line.is_empty() {
            continue;
        }
        let words = line.split(' ').collect::<Vec<_>>();
        let from = words[1].to_string();
        let to = words[3].to_string();
        let amount = part2[&from].min(words[5].parse::<i64>().unwrap());
        *part2.get_mut(&from).unwrap() -= amount;
        *part2.get_mut(&to).unwrap() += amount;
    }
    let mut balances = part2.values().copied().collect::<Vec<_>>();
    balances.sort();
    println!("  part 2 = {}", balances.iter().rev().take(3).sum::<i64>());

    let mut part3 = starting_balances.clone();
    let mut owes = HashMap::<String, Vec<(String, i64)>>::new();
    for line in sections[1].split('\n') {
        if line.is_empty() {
            continue;
        }
        let words = line.split(' ').collect::<Vec<_>>();
        let from = words[1].to_string();
        let to = words[3].to_string();
        let amount = words[5].parse::<i64>().unwrap();

        if amount > part3[&from] {
            // split into a paid + debt
            let can_pay = part3[&from];
            let debt = amount - can_pay;
            *part3.get_mut(&from).unwrap() = 0;
            *part3.get_mut(&to).unwrap() += can_pay;
            owes.entry(from.clone())
                .or_default()
                .push((to.clone(), debt));
        } else {
            // can pay full amount
            *part3.get_mut(&from).unwrap() -= amount;
            *part3.get_mut(&to).unwrap() += amount;
        }

        // println!("{line}:");
        // println!("  {part3:?}");
        // println!("  {owes:?}");

        // check the `owes` map and see if we can settle debts
        loop {
            let mut money_moved = false;
            for person in owes.iter_mut() {
                let mut amount = part3[person.0];
                // println!("  {} has {amount}", person.0);
                if amount == 0 {
                    continue;
                }

                for debt in person.1 {
                    let payment = debt.1.min(amount);
                    if payment == 0 {
                        continue;
                    }
                    debt.1 -= payment;
                    amount -= payment;
                    *part3.get_mut(&debt.0).unwrap() += payment;
                    *part3.get_mut(person.0).unwrap() -= payment;
                    money_moved = true;
                    // println!("  -> paying {payment} to {}", debt.0);
                    if amount == 0 {
                        break;
                    }
                }
            }
            if !money_moved {
                break;
            }
        }
        // println!("After Payments:");
        // println!("  {part3:?}");
        // println!("  {owes:?}");
        // println!("--");
    }
    let mut balances = part3.values().copied().collect::<Vec<_>>();
    balances.sort();
    println!("  part 3 = {}", balances.iter().rev().take(3).sum::<i64>());

    Ok(())
}
