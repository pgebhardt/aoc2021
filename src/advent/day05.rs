use futures::{pin_mut, prelude::*};
use logos::{Lexer, Logos};
use ndarray::{Array2, ArrayViewMut2};
use std::error::Error;

/// The token on the line segment input
#[derive(Logos, Debug, PartialEq)]
enum LineSegment {
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(i32),

    #[regex(",", logos::skip)]
    Comma,

    #[regex("->", logos::skip)]
    Arrow,

    #[regex(" +", logos::skip)]
    Whitespace,

    #[error]
    Error,
}

/// Helper to extract a coordinate pair from a [LineSegment]
fn get_number(segment: &mut Lexer<LineSegment>) -> Option<i32> {
    segment.next().and_then(|s| {
        if let LineSegment::Number(n) = s {
            Some(n)
        } else {
            None
        }
    })
}

/// Helper to render a line segment using Bresenham-Algorithm
fn render_line(mut grid: ArrayViewMut2<u32>, start: (i32, i32), end: (i32, i32)) {
    let (mut x1, mut y1) = start;
    let (x2, y2) = end;

    let dx = (x2 - x1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let dy = -(y2 - y1).abs();
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut e2;
    loop {
        grid[[x1 as usize, y1 as usize]] += 1;
        if x1 == x2 && y1 == y2 {
            break;
        }
        e2 = 2 * err;
        if e2 > dy {
            err += dy;
            x1 += sx;
        }
        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }
}

/// Executes the exercise of day 5
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u32; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in all line segments and add a one to each field
    // occupied by a vent
    let mut grid01: Array2<u32> = Array2::zeros((1000, 1000));
    let mut grid02: Array2<u32> = Array2::zeros((1000, 1000));
    while let Some(line) = input.try_next().await? {
        // parse the line segment
        let mut segment = LineSegment::lexer(&line);

        // extract start and end of line
        let start = (
            get_number(&mut segment).unwrap(),
            get_number(&mut segment).unwrap(),
        );
        // extract start and end of line
        let end = (
            get_number(&mut segment).unwrap(),
            get_number(&mut segment).unwrap(),
        );

        // render all lines in grid 2
        render_line(grid02.view_mut(), start, end);

        // render only horizontal/vertical lines in grid 1
        if start.0 == end.0 || start.1 == end.1 {
            render_line(grid01.view_mut(), start, end);
        }
    }

    // count fields with two overlapping lines
    let overlap01: u32 = grid01.iter().map(|e| if *e >= 2 { 1 } else { 0 }).sum();
    let overlap02: u32 = grid02.iter().map(|e| if *e >= 2 { 1 } else { 0 }).sum();

    Ok([overlap01, overlap02])
}
