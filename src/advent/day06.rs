use futures::{pin_mut, prelude::*};
use std::{collections::HashMap, error::Error};

/// Executes the exercise of day 6
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in initial population
    let initial: Vec<i32> = input
        .try_next()
        .await?
        .unwrap()
        .split(',')
        .map(|lf| lf.parse().unwrap())
        .collect();
    let mut population = HashMap::new();
    for timer in initial {
        *population.entry(timer).or_insert(0) += 1;
    }

    // grow population over time
    let mut eighty_days = 0;
    for i in 0..256 {
        // advance timer
        let mut next_population = HashMap::new();
        for (timer, count) in population.iter() {
            *next_population.entry(timer - 1).or_insert(0) = *count;
        }

        // grow population
        if let Some(&count) = next_population.get(&-1) {
            next_population.insert(8, count);
            *next_population.entry(6).or_insert(0) += count;
            next_population.remove(&-1);
        }
        population = next_population;

        if i == 79 {
            eighty_days = population.iter().map(|(_, count)| *count).sum();
        }
    }

    Ok([
        eighty_days,
        population.iter().map(|(_, count)| *count).sum(),
    ])
}
