use cgmath::{prelude::*, Vector2};
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
#[inline]
fn get_number(segment: &mut Lexer<LineSegment>) -> Option<i32> {
    segment.next().and_then(|s| {
        if let LineSegment::Number(n) = s {
            Some(n)
        } else {
            None
        }
    })
}

/// Helper to extract a 2d vector from a [LineSegment]
#[inline]
fn get_point(segment: &mut Lexer<LineSegment>) -> Option<Vector2<i32>> {
    get_number(segment).and_then(|x| get_number(segment).map(|y| Vector2::new(x, y)))
}

/// Helper to render a line segment using Bresenham-Algorithm
fn render_line(mut grid: ArrayViewMut2<u32>, start: Vector2<i32>, end: Vector2<i32>) {
    // get sloap and step width
    let delta = Vector2::new((end.x - start.x).abs(), -(end.y - start.y).abs());
    let step = Vector2::new((end.x - start.x).signum(), (end.y - start.y).signum());

    // render line
    let mut pos = start;
    let mut err = delta.sum();
    let mut e2;
    loop {
        grid[[pos.x as usize, pos.y as usize]] += 1;
        if pos == end {
            break;
        }
        e2 = 2 * err;
        if e2 > delta.y {
            err += delta.y;
            pos.x += step.x;
        }
        if e2 < delta.x {
            err += delta.x;
            pos.y += step.y;
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
        let start = get_point(&mut segment).unwrap();
        let end = get_point(&mut segment).unwrap();

        // render all lines in grid 2
        render_line(grid02.view_mut(), start, end);

        // render only horizontal/vertical lines in grid 1
        if start.x == end.x || start.y == end.y {
            render_line(grid01.view_mut(), start, end);
        }
    }

    // count fields with two overlapping lines
    let overlap01: u32 = grid01.iter().map(|e| if *e >= 2 { 1 } else { 0 }).sum();
    let overlap02: u32 = grid02.iter().map(|e| if *e >= 2 { 1 } else { 0 }).sum();

    Ok([overlap01, overlap02])
}
