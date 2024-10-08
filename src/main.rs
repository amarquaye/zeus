use std::env;
use std::fs;

fn main() {
    let arguments:Vec<String> = env::args().collect();

    if arguments.len() > 1 && arguments[1] == "create" {
        // Create an empty file(s) if it does not exist already 
        for file in arguments[2..].iter() {
            fs::File::create_new(file).expect("Failed to create file!");
        }

    } else if arguments.len() > 1 && arguments[1] == "cat" {
        // Display the contents of the file as a string 
        let file = fs::read_to_string(&arguments[2]).expect("Failed to read contents!");
        for lines in file.lines() {
            println!("{:#?}", lines.trim())
        }

    } else if arguments.len() > 1 && arguments[1] == "rm" {
        // Remove a file if it already exists 
        fs::remove_file(&arguments[2]).expect("Cannot remove file!");

    } else if arguments.len() > 1 && arguments[1] == "mkdir" {
        // Create an empty directory if it does not exist already
        for item in arguments[2..].iter() {
            fs::create_dir_all(item).expect("Failed to create directory!");
        }

    } else if arguments.len() > 1 && arguments[1] == "rmdir" {
        // Remove an empty directory
        fs::remove_dir(&arguments[2]).expect("Cannot remove directory!");

    } else if arguments.len() > 1 && arguments[1] == "stat" {
        // Display the stats for a given file or directory 
        let stats = fs::metadata(&arguments[2]).expect("Cannot get stats!");
        println!("{:#?}", stats);

    } else if arguments.len() > 1 && arguments[1] == "pwd" {
        // Display the current working directory. Similar to pwd in Unix or Linux systems 
        let cwd = env::current_dir().expect("Cannot get current working directory!");
        println!("{:?}", cwd.display());

    } else {
        println!("Incorrect command!");

    }
}