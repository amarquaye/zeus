use std::env;
use std::fs;

fn main() {
    let arguments:Vec<String> = env::args().collect();

    if arguments[1] == "touch" {

        // Create an empty file if it does not exit already 
        fs::File::create(&arguments[2]).expect("Failed to create file!");

    } else if arguments[1] == "cat" {
        // Display the contents of the files as a string 
        let file = fs::read_to_string(&arguments[2]).expect("Failed to read contents!");
        for lines in file.lines() {
            println!("{:#?}", lines.trim())
        }

    } else if arguments[1] == "rm" {
        // Remove a file if it already exists 
        fs::remove_file(&arguments[2]).expect("Cannot remove file!");

    } else if arguments[1] == "mkdir" {
        // Create an empty directory if it does not exist already
        fs::create_dir(&arguments[2]).expect("Cannot create directory!");

    } else if arguments[1] == "rmdir" {
        // Remove an empty directory
        fs::remove_dir(&arguments[2]).expect("Cannot remove directory!");

    } else if arguments[1] == "stat" {
        // Display the stats for a given file or directory 
        let stats = fs::metadata(&arguments[2]).expect("Cannot get stats!");
        println!("{:#?}", stats);

    } else if arguments[1] == "pwd" {
        // Display the current working directory. Similar to pwd in Unix or Linux systems 
        let cwd = env::current_dir().expect("Cannot get current working directory!");
        println!("{:?}", cwd.display());
    }
}