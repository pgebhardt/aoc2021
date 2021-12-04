use futures::prelude::*;
use std::{error::Error, path::Path};
use strum::{Display, EnumString};

mod day04;

/// The day of the AdventOfCode calender to execut
#[derive(Debug, EnumString, Display)]
pub enum CalenderDay {
    Four,
}

impl CalenderDay {
    /// Execute the exercise of the day
    pub async fn execute<E: Error + 'static>(
        &self,
        input: impl Stream<Item = Result<String, E>>,
    ) -> Result<Vec<u32>, Box<dyn Error>> {
        match self {
            Self::Four => day04::execute(input).await,
        }
    }

    /// Get the path to the input file
    pub fn input_path(&self) -> &Path {
        match self {
            Self::Four => "input4.txt",
        }
        .as_ref()
    }
}
