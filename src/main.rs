use clap::{Parser, Subcommand};
use colored::*;
use hashpass::{hash_password, verify_password};
use bcrypt::DEFAULT_COST;
mod file_handler;
use file_handler::{read_wordlist, FileError};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A simple CLI tool for password hashing and verification",
    long_about = "Hashpass allows you to securely hash passwords and verify them using bcrypt algorithm."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a bcrypt hash from a password string
    #[command(short_flag = 'g', visible_alias = "hash")]
    Generate {
        /// The password string to hash
        string: String,
        /// Number of rounds for hashing (default: 10, recommended: 10-12)
        #[arg(short = 'r', long = "rounds", default_value_t = DEFAULT_COST)]
        rounds: u32,
    },
    /// Verify if a password matches a bcrypt hash
    #[command(short_flag = 'c', visible_alias = "verify")]
    Check {
        /// The password string to verify
        string: String,
        /// The bcrypt hash to verify against
        hash: String,
    },
    /// Check a wordlist file against a hash
    #[command(short_flag = 'l', visible_alias = "list")]
    List {
        /// Path to the wordlist file
        #[arg(help = "Path to the wordlist file containing passwords to check")]
        wordlist: String,
        /// The bcrypt hash to verify against
        hash: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { string, rounds } => {
            match hash_password(&string, rounds) {
                Ok(hash) => println!("{}", hash),
                Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
            }
        }
        Commands::Check { string, hash } => {
            match verify_password(&string, &hash) {
                Ok(result) => {
                    if result {
                        println!("{}", "Password matches the hash".green());
                    } else {
                        println!("{}", "Password does not match the hash".red());
                    }
                }
                Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
            }
        }
        Commands::List { wordlist, hash } => {
            println!("{}", format!("Checking wordlist: {}", wordlist).yellow());
            match read_wordlist(&wordlist) {
                Ok(lines) => {
                    let mut found_match = false;
                    for line_result in lines {
                        match line_result {
                            Ok(password) => {
                                match verify_password(&password, &hash) {
                                    Ok(result) => {
                                        if result {
                                            println!("{}", format!("Found matching password: {}", password).green());
                                            found_match = true;
                                        }
                                    }
                                    Err(e) => eprintln!("{}", format!("Error verifying '{}': {}", password, e).red()),
                                }
                            }
                            Err(FileError::InvalidLine(msg)) => {
                                eprintln!("{}", format!("Warning: {}", msg).yellow());
                            }
                            Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
                        }
                    }
                    if !found_match {
                        println!("{}", "No matching password found in wordlist".yellow());
                    }
                }
                Err(e) => eprintln!("{}", format!("Error opening wordlist: {}", e).red()),
            }
        }
    }
}
