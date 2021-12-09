use futures::prelude::*;
use std::{error::Error, path::PathBuf};
use strum::{Display, EnumString};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

/// The day of the AdventOfCode calender to execut
#[derive(Debug, EnumString, Display)]
pub enum CalenderDay {
    #[strum(serialize = "01")]
    One,
    #[strum(serialize = "02")]
    Two,
    #[strum(serialize = "03")]
    Three,
    #[strum(serialize = "04")]
    Four,
    #[strum(serialize = "05")]
    Five,
    #[strum(serialize = "06")]
    Six,
    #[strum(serialize = "07")]
    Seven,
    #[strum(serialize = "08")]
    Eight,
    #[strum(serialize = "09")]
    Nine,
}

impl CalenderDay {
    /// Execute the exercise of the day
    pub async fn execute<E: Error + 'static>(
        &self,
        input: impl Stream<Item = Result<String, E>>,
    ) -> Result<[u64; 2], Box<dyn Error>> {
        match self {
            Self::One => day01::execute(input).await,
            Self::Two => day02::execute(input).await,
            Self::Three => day03::execute(input).await,
            Self::Four => day04::execute(input).await,
            Self::Five => day05::execute(input).await,
            Self::Six => day06::execute(input).await,
            Self::Seven => day07::execute(input).await,
            Self::Eight => day08::execute(input).await,
            Self::Nine => day09::execute(input).await,
        }
    }

    /// Get the path to the input file
    pub fn input_path(&self) -> PathBuf {
        format!("input{}.txt", self).into()
    }
}
