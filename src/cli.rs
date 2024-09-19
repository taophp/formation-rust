use clap::{Parser, Subcommand};
use txt::*;
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
    Reverse {
        #[clap(short, long, value_parser)]
        inputfile: Option<String>,
    },
    IsPalindrome {
        #[clap(short, long, value_parser)]
        inputfile: Option<String>,
    },
    RemoveVowels {
        #[clap(short, long, value_parser)]
        inputfile: Option<String>,
    },
    RleEncode {
        #[clap(short, long, value_parser)]
        inputfile: Option<String>,
    },
    ToCase {
        #[clap(short, long, value_parser)]
        case: String,
        inputfile: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Reverse { inputfile } => {
            let input = get_input(inputfile);
            if input.trim().is_empty() {
                eprintln!("No input provided");
                process::exit(1);
            }
            println!("{}", reverse(&input));
        },
        Commands::IsPalindrome { inputfile } => {
            let input = get_input(inputfile);
            if input.trim().is_empty() {
                eprintln!("No input provided");
                process::exit(1);
            }
            if is_palindrome(&input) {
                println!("true");
            } else {
                println!("false");
            }
        },
        Commands::RemoveVowels { inputfile } => {
            let input = get_input(inputfile);
            if input.trim().is_empty() {
                eprintln!("No input provided");
                process::exit(1);
            }
            println!("{}", remove_vowels(&input));
        },
        Commands::RleEncode { inputfile } => {
            let input = get_input(inputfile);
            if input.trim().is_empty() {
                eprintln!("No input provided");
                process::exit(1);
            }
            println!("{}", rle_encode(&input));
        },
        Commands::ToCase { case, inputfile } => {
            let input = get_input(inputfile);
            if input.trim().is_empty() {
                eprintln!("No input provided");
                process::exit(1);
            }
            let case_type = match case.as_str() {
                "snake" => TxtToCase::SnakeCase,
                "camel" => TxtToCase::CamelCase,
                "kebab" => TxtToCase::KebabCase,
                _ => {
                    eprintln!("Invalid case type: choose between snake, camel, kebab");
                    process::exit(1);
                }
            };
            println!("{}", to_case(&input, case_type));
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
