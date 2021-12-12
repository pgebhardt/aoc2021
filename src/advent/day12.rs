use futures::{pin_mut, prelude::*};
use std::{collections::HashMap, error::Error};

/// Some common cave names
const START: &str = "start";
const END: &str = "end";

/// A path trough the cave system
#[derive(Clone, Debug)]
struct Path {
    path: String,
    pos: String,
    histogram: HashMap<String, usize>,
    ended: bool,
    progressed: bool,
}

impl Path {
    /// Create a new path an the start of the cave system
    fn new() -> Self {
        let mut histogram = HashMap::new();
        histogram.insert(START.to_owned(), 1);

        Self {
            path: START.to_owned(),
            pos: START.to_owned(),
            histogram,
            ended: false,
            progressed: false,
        }
    }

    /// Progress the path to the new location
    fn progress(&mut self, pos: &str) {
        // ended paths cannot progress
        if self.ended {
            return;
        }

        // update position and total path
        self.pos = pos.to_owned();
        self.path.push('-');
        self.path.push_str(pos);
        self.progressed = true;

        // update histogramm of small caves
        if pos.chars().all(char::is_lowercase) {
            if let Some(cave) = self.histogram.get_mut(pos) {
                *cave += 1;
            } else {
                self.histogram.insert(pos.to_owned(), 1);
            }
        }

        // check, if path has ended
        if pos == END {
            self.ended = true;
        }
    }
}
/// Walk through all possible paths
fn walk_paths(caves: &HashMap<String, Vec<String>>, valid: impl Fn(&Path) -> bool) -> Vec<Path> {
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
            let path = paths[i].clone();
            for (j, con) in (&caves[&path.pos]).iter().enumerate() {
                // build new path and skip it, if it already exists
                let mut path = path.clone();
                path.progress(con);

                // verify path
                if !valid(&path) {
                    continue;
                }
                progressed = true;

                // replace the existing path or add new path to the vecotr
                if j == 0 {
                    paths[i] = path;
                } else {
                    paths.push(path);
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

    // filter paths that end at "end"
    paths.into_iter().filter(|p| p.ended).collect()
}

/// Validator for part 1
#[inline]
fn valid_part_1(path: &Path) -> bool {
    !path.histogram.iter().any(|(_, &v)| v > 1)
}

/// Validator for part 2
#[inline]
fn valid_part_2(path: &Path) -> bool {
    // verify hist
    let mut twice = false;
    for (con, &count) in &path.histogram {
        if con == START && count > 1 {
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
