use futures::{pin_mut, prelude::*};
use std::{collections::VecDeque, error::Error};

/// Executes the exercise of day 01
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // Iterate over each line and calculate metrics
    let mut previous = (None, None);
    let mut sum = 0;
    let mut count = (0, 0);
    let mut window = VecDeque::new();
    while let Some(line) = input.try_next().await? {
        // parse line value
        let line: u64 = line.parse()?;

        // check if value of line has increased
        if let Some(previous) = previous.0 {
            if line > previous {
                count.0 += 1;
            }
        }
        previous.0 = Some(line);

        // calculate sum of sliding window and put values into a queue
        // for a more efficient calculation
        window.push_back(line);
        sum += line;
        match window.len().cmp(&3) {
            std::cmp::Ordering::Greater => {
                sum -= window.pop_front().unwrap();
            }
            std::cmp::Ordering::Less => {
                continue;
            }
            _ => {}
        }

        // check if sum of window has increased
        if let Some(previous) = previous.1 {
            if sum > previous {
                count.1 += 1;
            }
        }
        previous.1 = Some(sum);
    }

    Ok([count.0, count.1])
}
