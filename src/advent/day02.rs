use futures::{pin_mut, prelude::*};
use logos::Logos;
use std::error::Error;

/// The commands expected in out input
#[derive(Logos, Debug, PartialEq)]
enum Command {
    #[token("forward")]
    Forward,

    #[token("down")]
    Down,

    #[token("up")]
    Up,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(u32),

    #[regex(" +", logos::skip)]
    Whitespace,

    #[error]
    Error,
}

/// A submarine
#[derive(Default)]
struct Submarine {
    position: u32,
    depth: u32,
    aim: u32,
}

impl Submarine {
    /// Execute the command of part 1
    fn execute_command_1(&mut self, command: &Command, param: u32) {
        match command {
            Command::Forward => self.position += param,
            Command::Down => self.depth += param,
            Command::Up => self.depth -= param,
            _ => {}
        }
    }

    /// Execute the command of part 2
    fn execute_command_2(&mut self, command: &Command, param: u32) {
        match command {
            Command::Forward => {
                self.position += param;
                self.depth += self.aim * param;
            }
            Command::Down => self.aim += param,
            Command::Up => self.aim -= param,
            _ => {}
        }
    }

    /// Caluclate the score of the submarine
    fn score(&self) -> u32 {
        self.position * self.depth
    }
}

/// Executes the exercise of day 02
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u64; 2], Box<dyn Error>> {
    pin_mut!(input);

    // Iterate over each line and update submarine position accordingly
    let mut submarines = (Submarine::default(), Submarine::default());
    while let Some(line) = input.try_next().await? {
        // parse the command
        let mut command = Command::lexer(&line);

        // execute command and update submarine state accordingly
        if let Some(dir) = command.next() {
            // extract the command parameter
            let param = if let Some(Command::Number(param)) = command.next() {
                param
            } else {
                continue;
            };

            // execute command
            submarines.0.execute_command_1(&dir, param);
            submarines.1.execute_command_2(&dir, param);
        }
    }

    Ok([submarines.0.score() as u64, submarines.1.score() as u64])
}
