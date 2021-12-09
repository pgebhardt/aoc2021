use futures::{pin_mut, prelude::*};
use ndarray::prelude::*;
use std::{collections::HashMap, error::Error};

/// Executes the exercise of day 9
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in the heightmap
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
        map.as_mut()
            .unwrap()
            .push_row(ArrayView::from(&row))
            .unwrap();
    }
    let map = map.unwrap();

    // Find the low points
    let mut lows = HashMap::new();
    for ((i, j), v) in map.indexed_iter() {
        if i > 0 && map[[i - 1, j]] <= *v {
            continue;
        }
        if i < map.shape()[0] - 1 && map[[i + 1, j]] <= *v {
            continue;
        }
        if j > 0 && map[[i, j - 1]] <= *v {
            continue;
        }
        if j < map.shape()[1] - 1 && map[[i, j + 1]] <= *v {
            continue;
        }
        lows.insert((i, j), *v);
    }
    let risk: u32 = lows.iter().map(|((_, _), v)| 1 + v).sum();

    // Find the basin for each point
    let mut basins = HashMap::new();
    for ((i, j), v) in map.indexed_iter() {
        // skip points with a 9
        if *v == 9 {
            continue;
        }

        // low points automatically belong to their basin
        if lows.contains_key(&(i, j)) {
            *basins.entry((i, j)).or_insert(0u32) += 1;
            continue;
        }

        let (mut x, mut y) = (i, j);
        loop {
            // exit, if x and y have reached a low point
            if lows.contains_key(&(x, y)) {
                *basins.entry((x, y)).or_insert(0u32) += 1;
                break;
            }

            // find the direction of steepest decline
            let mut dir = (x, y, *v);
            if x > 0 && map[[x - 1, y]] < dir.2 {
                dir = (x - 1, y, map[[x - 1, y]]);
            }
            if x < map.shape()[0] - 1 && map[[x + 1, y]] < dir.2 {
                dir = (x + 1, y, map[[x + 1, y]]);
            }
            if y > 0 && map[[x, y - 1]] < dir.2 {
                dir = (x, y - 1, map[[x, y - 1]]);
            }
            if y < map.shape()[1] - 1 && map[[x, y + 1]] < dir.2 {
                dir = (x, y + 1, map[[x, y + 1]]);
            }
            x = dir.0;
            y = dir.1;
        }
    }
    let mut basin_size: Vec<_> = basins.iter().map(|(_, size)| *size).collect();
    basin_size.sort_unstable();

    Ok([
        risk as u64,
        basin_size.into_iter().rev().take(3).product::<u32>() as u64,
    ])
}
