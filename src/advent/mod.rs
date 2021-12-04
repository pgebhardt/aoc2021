use futures::prelude::*;
use std::{error::Error, path::Path};
use strum::{Display, EnumString};

mod day01;
mod day02;
mod day04;

/// The day of the AdventOfCode calender to execut
#[derive(Debug, EnumString, Display)]
pub enum CalenderDay {
    #[strum(serialize = "01")]
    One,
    #[strum(serialize = "02")]
    Two,
    #[strum(serialize = "04")]
    Four,
}

impl CalenderDay {
    /// Execute the exercise of the day
    pub async fn execute<E: Error + 'static>(
        &self,
        input: impl Stream<Item = Result<String, E>>,
    ) -> Result<[u32; 2], Box<dyn Error>> {
        match self {
            Self::One => day01::execute(input).await,
            Self::Two => day02::execute(input).await,
            Self::Four => day04::execute(input).await,
        }
    }

    /// Get the path to the input file
    pub fn input_path(&self) -> &Path {
        match self {
            Self::One => "input01.txt",
            Self::Two => "input02.txt",
            Self::Four => "input04.txt",
        }
        .as_ref()
    }
}
