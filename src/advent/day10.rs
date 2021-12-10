use futures::{pin_mut, prelude::*};
use std::error::Error;

/// Helper to calculate the syntax error score for a character
#[inline]
fn syntax_error_score(char: char) -> u64 {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

/// Helper to calculate the autocompletion score for a character
#[inline]
fn autocompletion_score(char: char) -> u64 {
    match char {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}
/// Executes the exercise of day 10
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in lines and disregard corrupted ones
    let mut error_score = 0;
    let mut completion_scores = Vec::new();
    'line: while let Some(line) = input.try_next().await? {
        // verify line chunks
        let mut chunks = Vec::new();
        for char in line.chars() {
            match char {
                // open a new chunk
                '(' | '{' | '[' | '<' => chunks.push(char),
                // close a chunk
                _ => {
                    let chunk = chunks.pop();
                    match chunk {
                        Some('(') => {
                            if char != ')' {
                                error_score += syntax_error_score(char);
                                continue 'line;
                            }
                        }
                        Some('{') => {
                            if char != '}' {
                                error_score += syntax_error_score(char);
                                continue 'line;
                            }
                        }
                        Some('[') => {
                            if char != ']' {
                                error_score += syntax_error_score(char);
                                continue 'line;
                            }
                        }
                        Some('<') => {
                            if char != '>' {
                                error_score += syntax_error_score(char);
                                continue 'line;
                            }
                        }
                        _ => panic!(),
                    }
                }
            }
        }
        completion_scores.push(
            chunks
                .into_iter()
                .rev()
                .fold(0, |acc, c| acc * 5 + autocompletion_score(c)),
        );
    }
    completion_scores.sort_unstable();
    let completion_score = completion_scores[completion_scores.len() / 2];

    Ok([error_score, completion_score])
}
