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
    /// Concatenate FILE(s) to standard output.
    Cat {
        /// FILE(s) to concatenate.
        #[arg(required = true)]
        file: Vec<path::PathBuf>,
    },
    /// Create the FILE(s), if they do not already exist.
    Create {
        /// FILE(s) to create.
        #[arg(required = true)]
        file: Vec<path::PathBuf>,
    },
    /// Echo the STRING(s) to standard output.
    Echo {
        /// STRING(s) to echo.
        string: Vec<String>,

        /// Do not output the trailing newline.
        #[arg(short)]
        n: bool,
    },
    /// Search for PATTERNS in each FILE.
    ///
    /// Example: zeus grep 'hello world' src/main.rs main.py
    Grep {
        /// String pattern.
        #[arg(required = true)]
        patterns: String,

        /// FILE(s) to search for pattern.
        #[arg(required = true)]
        file: Vec<path::PathBuf>,
    },
    /// Create the DIRECTORY(ies), if they do not already exist.
    Mkdir {
        /// DIRECTORY(ies) to create.
        #[arg(required = true)]
        dir: Vec<path::PathBuf>,
    },
    /// Remove (unlink) the FILE(s).
    ///
    /// By default, rm does not remove directories.  Use the --recursive (-r)
    /// option to remove each listed directory, too, along with all of its contents.
    ///
    /// To remove a file whose name starts with a '-', for example '-bar',
    /// use one of these commands:
    ///
    /// rm -- -bar
    ///
    /// rm ./-bar
    Rm {
        /// FILE(s) to remove.
        #[arg(required = true)]
        file: Vec<path::PathBuf>,

        /// Remove DIRECTORIES and their contents recursively.
        #[arg(short, long)]
        recursive: bool,
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
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Cat { file }) => {
            // Display file content.
            for f in file {
                let f = fs::read_to_string(f)?;
                for line in f.lines() {
                    println!("{}", line);
                }
            }
            Ok(())
        }
        Some(Commands::Create { file }) => {
            // Create files.
            for f in file {
                fs::File::create_new(f)
                    .with_context(|| format!("Failed to create file: {:?}", f))?;
            }
            Ok(())
        }
        Some(Commands::Echo { string, n }) => {
            // Echo strings
            if *n {
                for s in string {
                    print!("{} ", s.trim());
                }
            } else {
                for s in string {
                    print!("{} ", s.trim());
                }
                println!();
            }
            Ok(())
        } // TODO: Add color to patterns that get matched and craete a recursive function to search directories for patterns and display the name of each file that matched.
        Some(Commands::Grep { patterns, file }) => {
            for f in file {
                if f.is_file() {
                    let contents = fs::read_to_string(f).with_context(|| format!("Failed to read the contents of {:?}", f))?;
                    for (number, line) in contents.lines().enumerate() {
                        if line.contains(patterns) {
                            println!("{}: {}", number + 1, line);
                        }
                    }
                } else if f.is_dir() {
                    if let Ok(entries) = fs::read_dir(f) {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                if entry.path().is_file() {
                                    let contents = fs::read_to_string(entry.path()).with_context(|| format!("Failed to read the contents of {:?}", entry.path()))?;
                                    for (number, line) in contents.lines().enumerate() {
                                        if line.contains(patterns) {
                                            println!("{}: {}", number + 1, line);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        Some(Commands::Mkdir { dir }) => {
            // Create directories
            for d in dir {
                fs::create_dir_all(d)
                    .with_context(|| format!("Failed to create directory: {:?}", d))?;
            }
            Ok(())
        }
        Some(Commands::Rm { file, recursive }) => {
            // Remove files.
            for f in file {
                if *recursive && fs::metadata(f)?.is_dir() {
                    fs::remove_dir_all(f).with_context(|| {
                        format!("Failed to remove directory recursively: {:?}", f)
                    })?;
                } else {
                    fs::remove_file(f)
                        .with_context(|| format!("Failed to remove file: {:?}", f))?;
                }
            }
            Ok(())
        }
        Some(Commands::Rmdir { dir }) => {
            // Remove directories.
            for d in dir {
                fs::remove_dir(d)
                    .with_context(|| format!("Failed to remove directory: {:?}", d))?;
            }
            Ok(())
        }
        Some(Commands::Stat { file }) => {
            // Display the stats for a given file or directory
            for f in file {
                let stats =
                    fs::metadata(f).with_context(|| format!("Cannot get stats for {:?}", f))?;
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

