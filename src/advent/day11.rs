use futures::{pin_mut, prelude::*};
use ndarray::prelude::*;
use std::error::Error;

/// Executes the exercise of day 11
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in the energy map
    let mut map = None;
    while let Some(line) = input.try_next().await? {
        // extract all integers of a row
        let row: Vec<u32> = line
            .chars()
            .map(|char| char.to_digit(10).unwrap())
            .collect();

        // construct map
        if map.is_none() {
            map = Some(Array2::zeros((0, row.len())));
        }

        // add row, or construct map
        map.as_mut().unwrap().push_row(ArrayView::from(&row))?;
    }
    let mut map = map.unwrap();

    // run the simulation for some steps
    let mut flashes_100 = 0;
    let mut all_flash = 0;
    let mut flashed: Array2<bool> = Array2::from_elem(map.raw_dim(), false);
    for i in 0.. {
        // reset the flashed marker map
        flashed.fill(false);

        // increase all energy levels by 1
        map += 1;

        loop {
            // flash octopuses with energy above 9
            let new_flashes: Vec<_> = map
                .indexed_iter()
                .zip(flashed.iter_mut())
                .filter(|((_, &e), f)| e > 9 && !**f)
                .map(|((index, _), f)| {
                    *f = true;
                    index
                })
                .collect();

            // increase the energy level of all octopuses adjacent to the flash
            for (i, j) in &new_flashes {
                let left = if *i > 0 { i - 1 } else { 0 };
                let right = (i + 2).min(map.shape()[0]);
                let top = if *j > 0 { j - 1 } else { 0 };
                let bottom = (j + 2).min(map.shape()[1]);
                map.slice_mut(s![left..right, top..bottom])
                    .map_inplace(|e| *e += 1);
            }

            // break the loop, if no new octopuses flashed
            if new_flashes.is_empty() {
                break;
            }
        }

        // reset energy of flashed octopuses
        map.iter_mut()
            .zip(flashed.iter())
            .filter(|(_, &f)| f)
            .for_each(|(e, _)| *e = 0);

        // count flashed octopuses
        let flashes = flashed.iter().filter(|f| **f).count() as u64;
        if i < 100 {
            flashes_100 += flashes;
        }

        // exit simulation, if all octopuses flash
        if flashes == map.shape().iter().product::<usize>() as u64 {
            all_flash = i + 1;
            break;
        }
    }

    Ok([flashes_100, all_flash])
}
