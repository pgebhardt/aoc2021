use cgmath::{prelude::*, Vector2};
use futures::{pin_mut, prelude::*};
use logos::{Lexer, Logos};
use ndarray::prelude::*;
use std::error::Error;

/// The token of the line segment input
#[derive(Logos, Debug, PartialEq)]
enum LineSegment {
    #[regex("[0-9]+,[0-9]+", parse_point)]
    Point(Vector2<i32>),

    #[regex("->", logos::skip)]
    Arrow,

    #[regex(" +", logos::skip)]
    Whitespace,

    #[error]
    Error,
}

/// Parse a string like (x,y) into a [Vector2]
#[inline]
fn parse_point(lex: &mut Lexer<LineSegment>) -> Option<Vector2<i32>> {
    let mut split = lex.slice().split(',');
    let x = split.next()?.parse().ok()?;
    let y = split.next()?.parse().ok()?;
    Some(Vector2::new(x, y))
}

/// Helper to extract a 2d vector from a [LineSegment]
#[inline]
fn get_point(segment: &mut Lexer<LineSegment>) -> Option<Vector2<i32>> {
    segment.next().and_then(|s| {
        if let LineSegment::Point(p) = s {
            Some(p)
        } else {
            None
        }
    })
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
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // read in all line segments and add a one to each field
    // occupied by a vent
    let mut grids = Array3::zeros((2, 1000, 1000));
    while let Some(line) = input.try_next().await? {
        // parse the line segment
        let mut segment = LineSegment::lexer(&line);

        // extract start and end of line
        let start = get_point(&mut segment).unwrap();
        let end = get_point(&mut segment).unwrap();

        // render all lines in grid 2
        render_line(grids.slice_mut(s![1, .., ..]), start, end);

        // render only horizontal/vertical lines in grid 1
        if start.x == end.x || start.y == end.y {
            render_line(grids.slice_mut(s![0, .., ..]), start, end);
        }
    }

    // count fields with two overlapping lines
    let overlap: Vec<_> = grids
        .axis_iter(Axis(0))
        .map(|grid| grid.iter().filter(|&e| *e >= 2).map(|_| 1).sum())
        .collect();

    Ok([overlap[0], overlap[1]])
}
