use clap::{Parser, Subcommand};
use std::{fs, path};
/// Command-line utility to mimic some essential Unix/Linux commands. Written in rust 🦀.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create an empty file if it does not exist.
    Create {
        /// File to create.
        file: Vec<path::PathBuf>,
    },
    /// Remove file.
    Rm {
        /// File to remove.
        file: Vec<path::PathBuf>,
    },
    /// Create an empty directory/folder.
    Mkdir {
        /// Directory to create.
        dir: Vec<path::PathBuf>,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Create { file }) => {
            for f in file {
                // Create files.
                fs::File::create_new(f).expect("Cannot create file!");
            }
        }
        Some(Commands::Rm { file }) => {
            for f in file {
                // Remove files.
                fs::remove_file(f).expect("Failed to remove file!");
            }
        }
        Some(Commands::Mkdir { dir }) => {
            for d in dir {
                // Create directories
                fs::create_dir_all(d).expect("Failed to create directory!");
            }
        }
        None => {
            println!("No command given!\nTry zeus --help for more info.");
        }
    }
}
