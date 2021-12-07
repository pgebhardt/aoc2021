use futures::{pin_mut, prelude::*};
use std::error::Error;

/// Executes the exercise of day 7
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in initial crab positions
    let crabs: Vec<i32> = input
        .try_next()
        .await?
        .unwrap()
        .split(',')
        .map(|lf| lf.parse().unwrap())
        .collect();

    // find aligned position with minimum total fuel consumption part1
    let mut x = crabs.iter().sum::<i32>() / crabs.len() as i32;
    let mut fuel = i32::MAX;
    let lr_inv = crabs.len() as i32;
    loop {
        // calculate loss and update horizontal position using simple gradient descent
        let grad: i32 = crabs.iter().map(|&c| -(c - x).signum() * x).sum();
        x -= grad.signum() * (grad.abs() / lr_inv).max(1);

        // calculate loss and update fuel or exit loop, if needed
        let loss: i32 = crabs.iter().map(|&c| (c - x).abs()).sum();
        if loss < fuel {
            fuel = loss;
        } else {
            break;
        }
    }

    // find aligned position with minimum total fuel consumption part2
    let mut fuel2 = i32::MAX;
    let lr_inv = (crabs.len() * crabs.len()) as i32;
    loop {
        // calculate loss and update horizontal position using simple gradient descent
        let grad: i32 = -crabs
            .iter()
            .map(|&c| (c - x).signum() * x * (c - x).abs())
            .sum::<i32>()
            / 2;
        x -= grad.signum() * (grad.abs() / lr_inv).max(1);

        // calculate loss and update fuel or exit loop, if needed
        let loss = crabs
            .iter()
            .map(|&c| (c - x).abs())
            .map(|d| d * (d + 1))
            .sum::<i32>()
            / 2;
        if loss < fuel2 {
            fuel2 = loss;
        } else {
            break;
        }
    }

    Ok([fuel as u64, fuel2 as u64])
}
