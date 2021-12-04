use futures::{pin_mut, prelude::*};
use std::error::Error;

#[derive(Default)]
struct BinaryDigit {
    zero: u32,
    one: u32,
}

impl BinaryDigit {
    fn most_common_digit(&self) -> u32 {
        if self.one > self.zero {
            1
        } else {
            0
        }
    }

    fn least_common_digit(&self) -> u32 {
        if self.zero > self.one {
            1
        } else {
            0
        }
    }
}

#[derive(Default)]
struct DiagnosticCode {
    digits: [BinaryDigit; 12],
    entries: Vec<u64>,
}

impl DiagnosticCode {
    fn add_entry(&mut self, entry: &str) {
        for (char, digit) in entry.chars().zip(self.digits.iter_mut()) {
            match char {
                '0' => digit.zero += 1,
                '1' => digit.one += 1,
                _ => continue,
            }
        }
        self.entries.push(u64::from_str_radix(entry, 2).unwrap());
    }

    fn gamma_rate(&self) -> u32 {
        self.digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, d)| d.most_common_digit() * 2u32.pow(i as u32))
            .sum()
    }

    fn epsilon_rate(&self) -> u32 {
        self.digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, d)| d.least_common_digit() * 2u32.pow(i as u32))
            .sum()
    }

    fn oxygen_generator_rating(&self) -> u64 {
        if self.entries.is_empty() {
            return 0;
        }

        let mut entries: Vec<_> = self.entries.iter().copied().enumerate().collect();
        let mut pos = 12;
        while entries.len() > 1 {
            pos -= 1;

            let (mut zero, mut one) = (0, 0);
            let pow = 2u64.pow(pos);
            println!("pow: {:b}", pow);
            for entry in entries.iter() {
                println!("{}: {:b}", entry.0 + 1, entry.1);
            }

            // get most common digit
            for (_, entry) in entries.iter() {
                if *entry >= pow {
                    one += 1;
                } else {
                    zero += 1;
                }
            }
            dbg!(zero, one);
            let mcd = if one >= zero { 1 } else { 0 };

            // filter entries
            entries = entries
                .into_iter()
                .filter_map(|entry| {
                    if mcd == 1 && entry.1 >= pow {
                        Some((entry.0, entry.1 - pow))
                    } else if mcd == 0 && entry.1 < pow {
                        Some((entry.0, entry.1))
                    } else {
                        None
                    }
                })
                .collect();

            dbg!(entries.len());
        }

        dbg!(self.entries[dbg!(entries[0].0)])
    }

    fn co2_scrubber_rating(&self) -> u64 {
        if self.entries.is_empty() {
            return 0;
        }

        let mut entries: Vec<_> = self.entries.iter().copied().enumerate().collect();
        let mut pos = 12;
        while entries.len() > 1 {
            pos -= 1;

            let (mut zero, mut one) = (0, 0);
            let pow = 2u64.pow(pos);
            println!("pow: {:b}", pow);
            for entry in entries.iter() {
                println!("{}: {:b}", entry.0 + 1, entry.1);
            }

            // get most common digit
            for (_, entry) in entries.iter() {
                if *entry >= pow {
                    one += 1;
                } else {
                    zero += 1;
                }
            }
            dbg!(zero, one);
            let mcd = if one < zero { 1 } else { 0 };

            // filter entries
            entries = entries
                .into_iter()
                .filter_map(|entry| {
                    if mcd == 1 && entry.1 >= pow {
                        Some((entry.0, entry.1 - pow))
                    } else if mcd == 0 && entry.1 < pow {
                        Some((entry.0, entry.1))
                    } else {
                        None
                    }
                })
                .collect();

            dbg!(entries.len());
        }

        dbg!(self.entries[dbg!(entries[0].0)])
    }
}

/// Executes the exercise of day 02
pub async fn execute<E: Error + 'static>(
    input: impl Stream<Item = Result<String, E>>,
) -> Result<[u32; 2], Box<dyn Error>> {
    pin_mut!(input);

    let mut diagnostics = DiagnosticCode::default();
    while let Some(line) = input.try_next().await? {
        diagnostics.add_entry(&line);
    }

    Ok([
        diagnostics.gamma_rate() * diagnostics.epsilon_rate(),
        (diagnostics.oxygen_generator_rating() * diagnostics.co2_scrubber_rating()) as u32,
    ])
}
