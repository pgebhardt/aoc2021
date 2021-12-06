use futures::{pin_mut, prelude::*};
use std::{error::Error, time::Instant};
use rayon::prelude::*;

/// Executes the exercise of day 6
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u32; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in initial population
    let mut population: Vec<i32> = input
        .try_next()
        .await?
        .unwrap()
        .split(',')
        .map(|lf| lf.parse().unwrap())
        .collect();

    // grow population over 80 days
    let mut eighty_days = 0;
    for i in 0..256 {
        dbg!(i);

        // advance inner timer
        let start = Instant::now();
        population.par_iter_mut().for_each(|lf| *lf -= 1);
        println!("advance inner timer: {:?}", start.elapsed());

        // count due fish
        let start = Instant::now();
        let due = population.par_iter().filter(|&lf| *lf < 0).count();
        println!("count due fish: {:?}", start.elapsed());
        let start = Instant::now();
        population
            .par_iter_mut()
            .filter(|lf| **lf < 0)
            .for_each(|lf| *lf = 6);
        println!("count due fish: {:?}", start.elapsed());

        // add new fish
        let start = Instant::now();
        population.append(&mut vec![8; due]);
        println!("add new fish: {:?}", start.elapsed());

        if i == 79 {
            eighty_days = population.len();
        }
    }

    Ok([eighty_days as u32, population.len() as u32])
}
