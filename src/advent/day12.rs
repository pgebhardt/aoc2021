use futures::{pin_mut, prelude::*};
use std::{collections::HashMap, error::Error};

/// Some common cave names
const START: &str = "start";
const END: &str = "end";

/// A path trough the cave system
#[derive(Clone, Debug)]
struct Path<'a> {
    pos: &'a str,
    histogram: HashMap<&'a str, usize>,
    ended: bool,
    progressed: bool,
}

impl<'a> Path<'a> {
    /// Create a new path an the start of the cave system
    fn new() -> Self {
        let mut histogram = HashMap::new();
        histogram.insert(START, 1);

        Self {
            pos: START,
            histogram,
            ended: false,
            progressed: false,
        }
    }

    /// Progress the path to the new location
    fn progress(&mut self, pos: &'a str) {
        // ended paths cannot progress
        if self.ended {
            return;
        }

        // update current position and mark path as progressed
        self.pos = pos;
        self.progressed = true;

        // update histogramm of small caves
        if pos.chars().all(char::is_lowercase) {
            *self.histogram.entry(pos).or_default() += 1;
        }

        // check, if path has ended
        if pos == END {
            self.ended = true;
        }
    }
}

/// Walk through all possible paths
fn walk_paths(
    caves: &HashMap<String, Vec<String>>,
    valid: impl Fn(&Path, &str) -> bool,
) -> Vec<Path> {
    // Start one path at the beginning
    let mut paths = vec![Path::new()];

    // walk paths as long as there is progress
    loop {
        // advance each path
        let mut progressed = false;
        for i in 0..paths.len() {
            // skip paths, which already ended
            if paths[i].ended {
                continue;
            }
            paths[i].progressed = false;

            // branch out to all possible connections
            let mut first = true;
            let path = paths[i].clone();
            for con in &caves[path.pos] {
                // verify path
                if !valid(&path, con) {
                    continue;
                }
                progressed = true;

                // progress and replace the existing path or add new path to the vecotr
                if first {
                    paths[i].progress(con);
                    first = false;
                } else {
                    paths.push(path.clone());
                    paths.last_mut().unwrap().progress(con);
                }
            }
        }

        // get rid of all paths, that did not progress at all
        paths.retain(|path| path.progressed || path.ended);

        // end if there is no progress
        if !progressed {
            break;
        }
    }

    paths
}

/// Validator for part 1
#[inline]
fn valid_part_1(path: &Path, cave: &str) -> bool {
    !path.histogram.contains_key(cave)
}

/// Validator for part 2
#[inline]
fn valid_part_2(path: &Path, cave: &str) -> bool {
    // verify hist
    if path.histogram.contains_key(cave) {
        // "start" can only be visited once
        if cave == START {
            return false;
        }

        // check, if there is another cave which was visited twice
        if path.histogram.values().any(|count| *count > 1) {
            return false;
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
