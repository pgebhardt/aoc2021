use std::{error::Error, path::PathBuf, str::FromStr};
use structopt::{clap::AppSettings::*, StructOpt};
use tokio::{fs::File, io::BufReader};
use tokio_util::codec::{FramedRead, LinesCodec};

mod advent;

#[derive(Debug, StructOpt)]
#[structopt(author, about, setting(ColoredHelp))]
struct Cli {
    day: advent::CalenderDay,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // parse the command line commands
    let cli = Cli::from_args();
    let day = cli.day;

    // Open the input file
    let path = PathBuf::from_str("./inputs")?.join(day.input_path());
    let file = BufReader::new(File::open(path).await?);

    // Wrap input file into a lines codec to extract each line
    let input = FramedRead::new(file, LinesCodec::new());

    // run the exercise of the day and print the results
    let res = day.execute(input).await?;

    // print result
    println!("Result of day {}:", day);
    for (i, r) in res.into_iter().enumerate() {
        println!("* Part {}: {}", i + 1, r);
    }

    Ok(())
}
