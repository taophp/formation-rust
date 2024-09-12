use clap::{Parser, Subcommand};
use calc::*;
use std::{process, fs, io};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)] // Read from Cargo
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  Check {
      #[clap(short, long, value_parser)]
      inputfile: Option<String>,
  },
  Calc {
      #[clap(short, long, value_parser)]
      inputfile: Option<String>,
  },
  Equal {
      #[clap(short, long, value_parser)]
      inputfile: Option<String>,
  },
  Generate24,
  Check24 {
      #[clap(short, long, value_parser)]
      inputfile: Option<String>,
  },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
      Commands::Check { inputfile } => {
          if !check(&get_input(inputfile)) {
            eprintln!("Invalid expression");
            process::exit(1)
          }
        },
        Commands::Calc { inputfile } => {
          match calc(&get_input(inputfile)) {
            Ok(result) => println!("{}", result),
            Err(_) => {
              eprintln!("Invalid expression");
              process::exit(1)
            }
          }
        },
        Commands::Equal { inputfile } => {
          let input = &get_input(inputfile);

          match input.lines().collect::<Vec<&str>>().as_slice() {
              [line1, line2, ..] => {
                  match are_equal(line1,line2) {
                    Ok(0) => println!("1"),
                    Ok(-1) => process::exit(1),
                    Ok(1) => process::exit(2),
                    Ok(_) => eprintln!("WTF ?"),
                    Err(err) => println!("{:?}",err)
                  }
              },
              _ => {
                eprintln!("Not enough arguments");
                process::exit(1)
              },
          }
        },
        Commands::Generate24 => {
          let digits = generate_24_challenge();
          println!("{} {} {} {}", digits[0], digits[1], digits[2], digits[3])
        },
        Commands::Check24 { inputfile } => {
          let input = &get_input(inputfile);

          match input.lines().collect::<Vec<&str>>().as_slice() {
              [line1, line2, ..] => {
                let numbers: Vec<u8> = input
                    .split_whitespace()
                    .filter_map(|s| s.parse::<u8>().ok())
                    .collect();
                match numbers.as_slice() {
                    &[a, b, c, d] if (1..=9).contains(&a) &&
                                      (1..=9).contains(&b) &&
                                      (1..=9).contains(&c) &&
                                      (1..=9).contains(&d) => {
                                        match check_24_challenge(&[a, b, c, d], &line2) {
                                          Ok(()) => (),
                                          Err(CalcError::ChallengeFalseItems) => {
                                            eprintln!("Number first line failed validation");
                                            process::exit(1)
                                          },
                                          Err(CalcError::UnsolvedChallenge) => {
                                            eprintln!("Unsolved challenge");
                                            process::exit(2)
                                          },
                                          _ => {
                                            eprintln!("Wtf ?");
                                            process::exit(4)
                                          }
                                        }
                                      },
                    _ => {
                      eprintln!("First line mismatch");
                      process::exit(5)
                    }
                }
              },
              _ => {
                eprintln!("Not enough arguments");
                process::exit(1)
              },
          }
        },
    }
}

/**
 * Read a file in a string or stdin if none given
 */
fn get_input(inputfile: &Option<String>) -> String {
    match inputfile {
        Some(filename) => fs::read_to_string(filename).expect("Input file unreadable"),
        None => {
            let mut input: String = "".to_string();
            let lines = io::stdin().lines();
            for line in lines {
               input.push_str(&line.unwrap());
               input.push_str("\n");
            }
            input
        }
    }
}
