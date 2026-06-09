use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version = "0.1",
    about = "File synchroniser",
    long_about = "A focused two-file sync tool with primary-side conflict handling"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sync {
        #[arg(short, long)]
        primary: PathBuf,

        #[arg(short, long)]
        secondary: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sync { primary, secondary } => {
            match std::fs::metadata(&primary) {
                Ok(metadata) => {
                    if !metadata.is_file() {
                        eprintln!("primary is not a legit file : {:#?}", primary);
                        return;
                    }
                }
                Err(error) => {
                    eprintln!("Could not read primary path {:#?}:{}", primary, error);
                    return;
                }
            }

            match std::fs::metadata(&secondary) {
                Ok(metadata) => {
                    if !metadata.is_file() {
                        eprintln!("Secondary path exists but is not a file: {:#?}", secondary);
                        return;
                    }
                }
                Err(error) => {
                    if error.kind() == std::io::ErrorKind::NotFound {
                        println!("Secondary file does not exist; it will be created")
                    }
                }
            }

            // now copy the file
            let bytes = std::fs::copy(&primary, &secondary).expect("Copying failed");
            println!("copied {} bytes", bytes);
        }
    }
}
