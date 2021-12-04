use futures::{pin_mut, prelude::*};
use ndarray::prelude::*;
use std::error::Error;

/// A board of the bingo game
#[derive(Default)]
struct BingoBoard {
    board: Array2<u32>,
    marked: Array2<bool>,
    won: bool,
}

impl BingoBoard {
    /// Create a new bingo board from the given set of input lines
    fn from_lines(
        lines: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Self, Box<dyn Error>> {
        // extract board
        let mut board = None;
        for line in lines {
            // read in all numbers of a line
            let line: Vec<_> = line
                .as_ref()
                .split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect();

            // append line to board
            board
                .get_or_insert_with(|| Array2::zeros((0, line.len())))
                .push_row(ArrayView::from(&line))?;
        }
        let board = board.unwrap();

        Ok(Self {
            marked: Array2::from_elem(board.raw_dim(), false),
            board,
            won: false,
        })
    }

    /// Apply the given bingo number to the board. Returns true if the board has won
    fn apply_draw(&mut self, draw: u32) -> bool {
        // Only continue to apply draws, if the board has not won yet
        if self.won {
            return true;
        }

        // find the index of the draw, if present
        let index = self.board.indexed_iter().fold(None, |res, (index, elem)| {
            // check if element is equal to draw
            res.or_else(|| if elem == &draw { Some(index) } else { None })
        });

        // check, if have a match and mark the field
        if let Some(index) = index {
            self.marked[index] = true;
        } else {
            return false;
        }

        // check if board has won
        let won = self.marked.rows().into_iter().fold(false, |res, row| {
            row.into_iter().fold(true, |r, e| r & e) | res
        });
        self.won = self.marked.columns().into_iter().fold(won, |res, col| {
            col.into_iter().fold(true, |r, e| r & e) | res
        });

        self.won
    }

    /// Calculate the score of the board
    fn score(&self) -> u32 {
        self.board
            .iter()
            .zip(self.marked.iter())
            .fold(0, |res, (e, m)| if !*m { res + *e } else { res })
    }
}

/// Executes the exercise of day 4
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u32; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read the first line to extract the drawn bingo numbers
    let draws: Vec<u32> = input
        .try_next()
        .await?
        .unwrap()
        .split(',')
        .map(|draw| draw.parse().unwrap())
        .collect();

    // read in bingo boards as long as empty lines are available
    let mut boards = Vec::new();
    while let Some(_) = input.try_next().await? {
        // read 5 lines and generate a board
        let lines: Vec<_> = input.by_ref().take(5).try_collect().await?;
        boards.push(BingoBoard::from_lines(lines)?);
    }

    // apply the drawn numbers to each board and find
    let mut first = None;
    let mut last = None;
    for draw in draws {
        for board in &mut boards {
            // apply draw and if the board has won, calculate final score
            if board.apply_draw(draw) {
                let score = board.score() * draw;
                if first.is_none() {
                    first = Some(score);
                }
                last = Some(score);
            }
        }

        // Throw away boards that have already won
        boards.retain(|b| !b.won);
    }

    Ok([first.unwrap(), last.unwrap()])
}
