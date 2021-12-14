use futures::{pin_mut, prelude::*};
use std::{collections::HashMap, error::Error};

/// Executes the exercise of day 13
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in the polymere template
    let poly = input.try_next().await?.unwrap();

    // read in the insertion rules
    input.try_next().await?;
    let mut rules = HashMap::new();
    while let Some(line) = input.try_next().await? {
        let mut rule = line.split(" -> ");
        rules.insert(
            rule.next().unwrap().to_owned(),
            rule.next().unwrap().chars().next().unwrap(),
        );
    }

    // construct pair and character histogram
    let mut pairs = HashMap::new();
    let mut chars = HashMap::new();
    for i in 0..poly.len() - 1 {
        let pair: String = poly.chars().skip(i).take(2).collect();
        *pairs.entry(pair).or_insert(0usize) += 1;
    }
    for char in poly.chars() {
        *chars.entry(char).or_insert(0usize) += 1;
    }
    dbg!(&pairs);
    dbg!(&chars);

    // apply rules
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..40 {
        let mut new_pairs = pairs.clone();
        for (pair, res) in &rules {
            // remove current pair
            let occ = if let Some(o) = pairs.get(pair) {
                *o
            } else {
                continue;
            };

            // construct new pairs
            let pair1 = format!("{}{}", pair.chars().next().unwrap(), res);
            let pair2 = format!("{}{}", res, pair.chars().nth(1).unwrap());

            // update hists
            *new_pairs.get_mut(pair).unwrap() -= occ;
            *new_pairs.entry(pair1).or_insert(0usize) += occ;
            *new_pairs.entry(pair2).or_insert(0usize) += occ;
            *chars.entry(*res).or_insert(0usize) += occ;
        }
        pairs = new_pairs;

        // update stats
        if i == 9 || i == 39 {
            // find least and most occuring character
            let mut occurences: Vec<_> = chars.values().collect();
            occurences.sort_unstable();
            if i == 9 {
                part1 = occurences[occurences.len() - 1] - occurences[0];
            } else if i == 39 {
                part2 = occurences[occurences.len() - 1] - occurences[0];
            }
        }
    }

    Ok([part1 as u64, part2 as u64])
}
