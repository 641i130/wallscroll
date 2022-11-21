use colored::Colorize;
use std::{env, io};
use walkdir::WalkDir;

use std::fs::File;
use std::path::Path;
use serde::{Serialize};

pub struct IMG {
    path: String,
}


fn main() {
    // READ IN ARGUMENTS
    let mut arg_path: String = String::new();
    let args: Vec<String> = env::args().collect();
    let mut i: usize = 0; // Argument counter
    for arg in &args {
        // for each argument
        match arg.as_ref() {
            "-path" => {
                arg_path = args[i + 1].parse::<String>().unwrap();
                println!("{}", format!("arg_path : {}", arg_path).bright_purple());
                i += 1;
            }
            "-h" => {
                eprintln!("This program is a recursive image randomizer. It shuffles the images it finds in a given folder by default using a config file listing every image it found.\nExample Usage:\n./wallscroll -path './folder-with-images' -shuffle\n(shuffle is on by default)");
                std::process::exit(0); // exit without error
            }
            _ => i += 1,
        }
    }
    if arg_path == "" {
        eprintln!("Please provide a folder path using the '-path' argument.");
        std::process::exit(1); //exit with error
    }
    // Parse image list into a vec!
    // Maybe convert to json and write to file?
    //let files: Vec<String> = WalkDir::new("./md").into_iter().filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file()).map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_owned()).collect();
    let files: Vec<String> = WalkDir::new(&arg_path)
        .into_iter()
        .filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file())
        .filter(|e| (format!("{:?}",e.as_ref().unwrap()).contains(".jpeg")||format!("{:?}",e.as_ref().unwrap()).contains(".jpg")||format!("{:?}",e.as_ref().unwrap()).contains(".png")||format!("{:?}",e.as_ref().unwrap()).contains(".pnm")||format!("{:?}",e.as_ref().unwrap()).contains(".tiff")||format!("{:?}",e.as_ref().unwrap()).contains(".bmp")||format!("{:?}",e.as_ref().unwrap()).contains(".gif"))) 
        // Filter to be any type of image! FEH supports these so it will use them:
        // jpeg, png, pnm, tiff, bmp, gif (first frame only)
        .map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_owned())
        .collect();
    dbg!(&files); // now we need to write this to a config or file of sometype
}
