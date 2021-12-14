use futures::{pin_mut, prelude::*};
use ndarray::prelude::*;
use std::error::Error;

/// Executes the exercise of day 13
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in the dot locations
    let mut dots = Vec::new();
    let mut lines = input
        .by_ref()
        .try_take_while(|line| future::ok(!line.is_empty()));
    let (mut x_max, mut y_max) = (0, 0);
    while let Some(line) = lines.try_next().await? {
        let mut split = line.split(',');
        let x: usize = split.next().unwrap().parse().unwrap();
        let y: usize = split.next().unwrap().parse().unwrap();

        x_max = x.max(x_max);
        y_max = y.max(y_max);
        dots.push((x, y));
    }

    // build the map by marking the dots
    let mut map = Array2::from_elem((x_max + 1, y_max + 1), false);
    for (x, y) in dots {
        map[[x, y]] = true;
    }

    // perform folding instructions
    let mut first = None;
    while let Some(line) = input.try_next().await? {
        let mut fold = line.split_whitespace().nth(2).unwrap().split('=');
        let axis = fold.next().unwrap();
        let pos: usize = fold.next().unwrap().parse().unwrap();

        // execute split
        match axis {
            "x" => {
                // construct a new map with the correct dimensions
                let mut new = Array2::from_elem((pos, map.shape()[1]), false);

                // set dots
                new.slice_mut(s![0..pos, ..])
                    .assign(&map.slice(s![0..pos, ..]));
                new.slice_mut(s![0..pos, ..])
                    .into_iter()
                    .zip(map.slice(s![pos + 1..;-1, ..]))
                    .for_each(|(n, m)| *n |= *m);
                map = new;
            }
            "y" => {
                // construct a new map with the correct dimensions
                let mut new = Array2::from_elem((map.shape()[0], pos), false);

                // set dots
                new.slice_mut(s![.., 0..pos])
                    .assign(&map.slice(s![.., 0..pos]));
                new.slice_mut(s![.., 0..pos])
                    .into_iter()
                    .zip(map.slice(s![.., pos + 1..;-1]))
                    .for_each(|(n, m)| *n |= *m);
                map = new;
            }
            _ => unreachable!(),
        }

        if first.is_none() {
            first = Some(map.iter().filter(|e| **e).count());
        }
    }

    // Print the code image
    println!("--- Thermal camera code ---");
    map = map.t().to_owned();
    for row in map.rows() {
        for e in row {
            if *e {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("--- end ---\n");

    Ok([first.unwrap() as u64, 0])
}
