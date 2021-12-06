use futures::{pin_mut, prelude::*};
use std::error::Error;

/// Executes the exercise of day 6
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in initial population
    let mut population = [0u64; 10];
    input
        .try_next()
        .await?
        .unwrap()
        .split(',')
        .map(|lf| lf.parse().unwrap())
        .for_each(|timer: usize| population[timer] += 1);

    // grow population over time
    let mut eighty_days = 0;
    for i in 0..256 {
        // grow population
        population[7] += population[0];
        population[9] = population[0];

        // advance timer
        for i in 0..9 {
            population[i] = population[i + 1];
        }
        population[9] = 0;

        // capture total population after 80 days
        if i == 79 {
            eighty_days = population.iter().sum();
        }
    }

    Ok([eighty_days, population.iter().sum()])
}
