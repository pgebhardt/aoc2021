use futures::{pin_mut, prelude::*};
use std::{collections::HashMap, error::Error};

/// Walk through all possible paths
fn walk_paths(caves: &HashMap<String, Vec<String>>, valid: impl Fn(&str) -> bool) -> Vec<String> {
    // Start one path at the beginning
    let mut paths = vec!["start".to_owned()];

    // walk paths as long as there is progress
    let mut progress = Vec::new();
    loop {
        progress.clear();

        // advance each path
        for i in 0..paths.len() {
            // skip paths, which already ended
            if paths[i].ends_with("end") {
                continue;
            }

            // get current position in cave system
            let pos = paths[i].split('-').last().unwrap().to_owned();

            // branch out to all possible connections
            let history = paths[i].clone();
            for (j, con) in (&caves[&pos]).into_iter().enumerate() {
                // build new path and skip it, if it already exists
                let path = format!("{}-{}", history, con);
                if paths.contains(&path) {
                    continue;
                }

                // verify path
                if !valid(&path) {
                    continue;
                }

                // walk each path to the connection, if path not already exists
                if j == 0 {
                    paths[i] = path;
                    progress.push(i);
                } else {
                    paths.push(path);
                    progress.push(paths.len() - 1);
                }
            }
        }

        // get rid of all paths, that did not progress at all
        let mut index = 0usize;
        paths.retain(|path| {
            index += 1;
            progress.contains(&(index - 1)) || path.ends_with("end")
        });

        // end if there is no progress
        if progress.is_empty() {
            break;
        }
    }

    // filter paths that end at "end"
    paths.into_iter().filter(|p| p.ends_with("end")).collect()
}

/// Validator for part 1
#[inline]
fn valid_part_1(path: &str) -> bool {
    // build histogram
    let mut hist = HashMap::new();
    for pos in path
        .split('-')
        .filter(|c| c.chars().all(|c| c.is_lowercase()))
    {
        *hist.entry(pos.to_owned()).or_insert(0i32) += 1;
    }

    !hist.into_values().any(|v| v > 1)
}

/// Validator for part 2
#[inline]
fn valid_part_2(path: &str) -> bool {
    // build histogram
    let mut hist = HashMap::new();
    for pos in path
        .split('-')
        .filter(|c| c.chars().all(|c| c.is_lowercase()))
    {
        *hist.entry(pos.to_owned()).or_insert(0i32) += 1;
    }

    // verify hist
    let mut twice = false;
    for (con, count) in hist {
        if con == "start" && count > 1 {
            return false;
        }
        if count > 2 {
            return false;
        }
        if count > 1 {
            if twice {
                return false;
            } else {
                twice = true;
            }
        }
    }

    true
}

/// Executes the exercise of day 12
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in the cave layout
    let mut caves = HashMap::new();
    while let Some(line) = input.try_next().await? {
        // get cave connections
        let mut connection = line.split('-');
        let start = connection.next().unwrap();
        let end = connection.next().unwrap();

        // add connection to mapping
        caves
            .entry(start.to_owned())
            .or_insert_with(Vec::new)
            .push(end.to_owned());
    }

    // build a map of all possible cave connections
    let mut connections = HashMap::new();
    for (cave, conn) in caves {
        for c in conn {
            connections
                .entry(cave.clone())
                .or_insert_with(Vec::new)
                .push(c.clone());
            connections
                .entry(c.clone())
                .or_insert_with(Vec::new)
                .push(cave.clone());
        }
    }

    // walk through the cave system and count all distinct paths
    let paths1 = walk_paths(&connections, valid_part_1);
    let paths2 = walk_paths(&connections, valid_part_2);

    Ok([paths1.len() as u64, paths2.len() as u64])
}
