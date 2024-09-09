use clap::{Parser, Subcommand};
use mogulid::{Mogulid,MogulidError,mogulid_merge,mogulid_allow_update};
use std::{process, fs, io};

/// Provides a CLI interface to handle Mogulid
#[derive(Parser)]
#[clap(author, version, about, long_about = None)] // Read from Cargo
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Provides a new Mogulid JSON serialized
    Getnew,
    /// Provides an updated history of a Mogulid given on stdin
    Update {
        /// File containing the JSON formatted Mogulid
        #[clap(short, long, value_parser)]
        inputfile: Option<String>,
        /// Filename where to record the updated Mogulid, in the JSON format
        #[clap(short, long, value_parser)]
        outputfile: Option<String>,
    },
    /// Given two JSON formatted Mogulid, check if the first could be updated with the second
    Allowmerge {
        /// File containing the JSON array of the Mogulid to merge
        #[clap(short, long, value_parser)]
        inputfile: Option<String>,
    },
    /// Given two JSON formatted Mogulid, update the first with the second
    Merge {
        /// File containing the JSON array of the Mogulid to merge
        #[clap(short, long, value_parser)]
        inputfile: Option<String>,
        /// File where to write the merged Mogulid JSON
        #[clap(short, long, value_parser)]
        outputfile: Option<String>,
        /// Merge without checking if the merge should be allowed
        #[clap(short, long, action)]
        force: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Getnew => {
            let mogulid = Mogulid::new_start();
            println!("{}", serde_json::to_string(&mogulid).unwrap())
        },
        Commands::Update { inputfile, outputfile } => {
            let mut mogulid: Mogulid = serde_json::from_str(&get_input(inputfile)).unwrap();
            mogulid.define_new_state();
            put_output(outputfile, serde_json::to_string(&mogulid).unwrap())
        },
        Commands::Allowmerge { inputfile } => {
            let mogulids: [Mogulid; 2] = serde_json::from_str(&get_input(inputfile)).unwrap();
            match mogulid_allow_update(&mogulids[0], &mogulids[1]) {
                Ok(_) => (),
                Err(MogulidError::ChallengerOlderThanOriginal) => {
                    eprintln!("Challenger older than the original");
                    process::exit(1)
                },
                Err(MogulidError::Conflict) => {
                  eprintln!("The two versions are in conflict");
                  process::exit(2)
                },
                Err(MogulidError::NotVersionsOfTheSameObject) => {
                  eprintln!("The two versions does correspond to the same object");
                  process::exit(3)
                },
                Err(MogulidError::IdenticalVersions) => {
                  eprintln!("The two versions are identical");
                  process::exit(4)
                },
            }
        },
        Commands::Merge { inputfile, outputfile, force } => {
            let mogulids: [Mogulid; 2] = serde_json::from_str(&get_input(inputfile)).unwrap();
            match mogulid_allow_update(&mogulids[0], &mogulids[1]) {
                Ok(_) => put_output(outputfile, serde_json::to_string(&mogulid_merge(&mogulids[0], &mogulids[1])).unwrap()),
                Err(_) => {
                    if *force {
                        put_output(outputfile, serde_json::to_string(&mogulid_merge(&mogulids[0], &mogulids[1])).unwrap())
                    } else {
                        eprintln!("Update not allowed, run 'mogulid-cli allowmerge' to know why");
                        process::exit(1)
                    }
                }
            }
        },
    }
}

/**
 * Write in a file or stdout if none given
 */
fn put_output(outputfile: &Option<String>, output: String) {
    match outputfile {
        Some(filename) => fs::write(filename, output).expect("Output file unwritable"),
        None => println!("{}", output)
    };
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
            }
            input
        }
    }
}
