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

/// Helper to determine if a pattern equals another pattern
#[inline]
fn equals_pattern(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

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
    let mut hist = [0u64; 9];
    let mut output = 0u64;
    while let Some(line) = input.try_next().await? {
        let mut decoder = HashMap::new();
        let split: Vec<_> = line.split(" | ").map(|p| p.to_owned()).collect();

        // take the observed numbers and decode the pattern
        let mut observed = HashMap::new();
        for pattern in split[0].split_whitespace() {
            observed
                .entry(pattern.len())
                .or_insert(vec![])
                .push(pattern.to_owned());
        }

        // decode the easy ones first
        // One
        decoder.insert(observed[&2][0].clone(), 1);

        // Four
        decoder.insert(observed[&4][0].clone(), 4);

        // Seven
        decoder.insert(observed[&3][0].clone(), 7);

        // Eight
        decoder.insert(observed[&7][0].clone(), 8);

        // Three
        let three = observed[&5]
            .iter()
            .find(|p| contains_pattern(p, &observed[&2][0]))
            .unwrap();
        decoder.insert(three.to_owned(), 3);

        // Six
        let six = observed[&6]
            .iter()
            .find(|p| !contains_pattern(p, &observed[&3][0]))
            .unwrap();
        decoder.insert(six.to_owned(), 6);

        // Nine
        let nine = observed[&6]
            .iter()
            .filter(|&p| p != six)
            .find(|p| contains_pattern(p, three))
            .unwrap();
        decoder.insert(nine.to_owned(), 9);

        // Zero
        let zero = observed[&6]
            .iter()
            .find(|&p| p != six && p != nine)
            .unwrap();
        decoder.insert(zero.to_owned(), 0);

        // Two
        let two = observed[&5]
            .iter()
            .filter(|&p| p != three)
            .find(|p| common_character_count(p, &observed[&4][0]) == 2)
            .unwrap();
        decoder.insert(two.to_owned(), 2);

        // Five
        let five = observed[&5]
            .iter()
            .find(|&p| p != three && p != two)
            .unwrap();
        decoder.insert(five.to_owned(), 5);

        // take the input pattern
        let input_pattern = &split[1];

        // put the length of each pattern in a hist
        let mut res = 0u64;
        for (i, pattern) in input_pattern.split_whitespace().enumerate() {
            hist[pattern.len()] += 1;

            for (value, digit) in decoder.iter() {
                if equals_pattern(value, pattern) {
                    res += digit * 10u64.pow(3 - i as u32);
                }
            }
        }
        output += res;
    }

    Ok([hist[2] + hist[3] + hist[4] + hist[7], output])
}
