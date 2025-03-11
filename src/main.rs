use clap::{Parser, Subcommand};
use hashpass::{hash_password, verify_password};
use bcrypt::DEFAULT_COST;

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
    #[command(short_flag = 's', visible_alias = "hash")]
    String {
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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::String { string, rounds } => {
            match hash_password(&string, rounds) {
                Ok(hash) => println!("{}", hash),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Check { string, hash } => {
            match verify_password(&string, &hash) {
                Ok(result) => {
                    if result {
                        println!("Password matches the hash");
                    } else {
                        println!("Password does not match the hash");
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
}
