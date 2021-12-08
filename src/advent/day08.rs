use futures::{pin_mut, prelude::*};
use std::{collections::HashMap, error::Error};

/// Helper to determine if a pattern contains another pattern
#[inline]
fn contains_pattern(a: &str, b: &str) -> bool {
    for char in b.chars() {
        if !a.contains(char) {
            return false;
        }
    }

    true
}

/// Get the number of common characters of two patterns
#[inline]
fn common_character_count(a: &str, b: &str) -> usize {
    let mut res = 0;
    for char in b.chars() {
        if a.contains(char) {
            res += 1;
        }
    }
    res
}

/// Executes the exercise of day 8
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in the input pattern hist
    let mut hist = [0u64; 10];
    let mut sum = 0u64;
    while let Some(line) = input.try_next().await? {
        let split: Vec<_> = line.split(" | ").map(|p| p.to_owned()).collect();

        // take the observed numbers and decode the pattern
        let mut observed: HashMap<usize, Vec<String>> = HashMap::new();
        for pattern in split[0].split_whitespace() {
            // make sure each pattern is sorted
            let mut pattern: Vec<char> = pattern.chars().collect();
            pattern.sort_unstable();
            observed
                .entry(pattern.len())
                .or_insert_with(Vec::new)
                .push(pattern.into_iter().collect());
        }

        // decode the easy ones first
        let mut decoder = HashMap::new();

        // One (only pattern with 2 elements)
        decoder.insert(observed[&2][0].clone(), 1);

        // Four (only pattern with 4 elements)
        decoder.insert(observed[&4][0].clone(), 4);

        // Seven (only pattern with 3 elements)
        decoder.insert(observed[&3][0].clone(), 7);

        // Eight (only pattern with 7 elements)
        decoder.insert(observed[&7][0].clone(), 8);

        // Three (only pattern with 5 elements, which contains the one)
        let three = observed[&5]
            .iter()
            .find(|p| contains_pattern(p, &observed[&2][0]))
            .unwrap();
        decoder.insert(three.to_owned(), 3);

        // Six (only pattern with 6 elements, which does not contain the seven)
        let six = observed[&6]
            .iter()
            .find(|p| !contains_pattern(p, &observed[&3][0]))
            .unwrap();
        decoder.insert(six.to_owned(), 6);

        // Nine (only pattern with 6 elements, which is not the six and contains the three)
        let nine = observed[&6]
            .iter()
            .filter(|&p| p != six)
            .find(|p| contains_pattern(p, three))
            .unwrap();
        decoder.insert(nine.to_owned(), 9);

        // Zero (only pattern with 6 elements, which is neither the six nor the nine)
        let zero = observed[&6]
            .iter()
            .find(|&p| p != six && p != nine)
            .unwrap();
        decoder.insert(zero.to_owned(), 0);

        // Two (only pattern with 5 elements, which has two common elements with four)
        let two = observed[&5]
            .iter()
            .filter(|&p| p != three)
            .find(|p| common_character_count(p, &observed[&4][0]) == 2)
            .unwrap();
        decoder.insert(two.to_owned(), 2);

        // Five (only pattern with 5 elements, which neither the three nor the two)
        let five = observed[&5]
            .iter()
            .find(|&p| p != three && p != two)
            .unwrap();
        decoder.insert(five.to_owned(), 5);

        // take the input pattern and make sure each entry is sorted
        let input_pattern = split[1].split_ascii_whitespace().map(|p| {
            let mut p: Vec<char> = p.chars().collect();
            p.sort_unstable();
            p.into_iter().collect::<String>()
        });

        // put the length of each pattern in a hist
        for (i, pattern) in input_pattern.rev().enumerate() {
            // decode digit
            let digit = *decoder.get(&pattern).unwrap();

            // update histogram and total sum
            sum += digit * 10u64.pow(i as u32);
            hist[digit as usize] += 1;
        }
    }

    Ok([hist[1] + hist[4] + hist[7] + hist[8], sum])
}
