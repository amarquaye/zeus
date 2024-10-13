use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path};

/// Command-line utility to mimic some Unix/Linux commands. Written in rust 🦀.
///
/// Zeus is a tool that is being written as I learn the rust programming language.
/// Rust is a great systems programming language for writing cli applications.
/// Commands in zeus are on par with(in terms of execution speed) and surprisingly faster in some cases than the default commands written in C.
///
/// Developed by Jesse Amarquaye.
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Subcommand)]
enum Commands {
    /// Create the FILE(s), if they do not already exist.
    Create {
        /// FILE(s) to create.
        #[arg(required = true)]
        file: Vec<path::PathBuf>,
    },
    /// Remove (unlink) the FILE(s).
    Rm {
        /// FILE(s) to remove.
        #[arg(required = true)]
        file: Vec<path::PathBuf>,
    },
    /// Create the DIRECTORY(ies), if they do not already exist.
    Mkdir {
        /// DIRECTORY(ies) to create.
        #[arg(required = true)]
        dir: Vec<path::PathBuf>,
    },
    /// Remove the DIRECTORY(ies), if they are empty.
    Rmdir {
        /// DIRECTORY(ies) to remove.
        #[arg(required = true)]
        dir: Vec<path::PathBuf>,
    },
    /// Display file or file system status.
    Stat {
        /// File or file system to display status. 
        #[arg(required = true)]
        file: Vec<path::PathBuf>,
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Create { file }) => {
            for f in file {
                // Create files.
                fs::File::create_new(f).with_context(|| format!("Failed to create file: {:?}", f))?;
            }
            Ok(())
        }
        Some(Commands::Rm { file }) => {
            for f in file {
                // Remove files.
                fs::remove_file(f).with_context(|| format!("Failed to remove file: {:?}", f))?;
            }
            Ok(())
        }
        Some(Commands::Mkdir { dir }) => {
            for d in dir {
                // Create directories
                fs::create_dir_all(d).with_context(|| format!("Failed to create directory: {:?}", d))?;
            }
            Ok(())
        }
        Some(Commands::Rmdir { dir }) => {
            for d in dir {
                // Remove directories.
                fs::remove_dir(d).with_context(|| format!("Failed to remove directory: {:?}", d))?;
            }
            Ok(())
        }
        Some(Commands::Stat { file }) => {
            for f in file {
                // Display the stats for a given file or directory
                let stats = fs::metadata(f).with_context(|| format!("Cannot get stats for {:?}", f))?;
                println!("{:#?}\n", stats);
            }
            Ok(())
        }
        None => {
            println!("zeus: missing command operand!\nTry 'zeus --help' for more information.");
            Ok(())
        }
    }
}
