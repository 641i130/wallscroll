use colored::Colorize;
use std::{env,fs};
use walkdir::WalkDir;
use rand::prelude::SliceRandom;
use dirs::home_dir;
use std::fs::{File,OpenOptions};
use std::io::{BufReader,BufRead,Write};
use rand::thread_rng;

fn main() {
    // READ IN ARGUMENTS
    let mut arg_path: String = String::new();
    let mut arg_name: String = String::new();
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
            "-name" => {
                arg_name = args[i + 1].parse::<String>().unwrap();
                println!("{}", format!("arg_name : {}", arg_name).bright_purple());
                i += 1;
            }
            "-h" => {
                eprintln!("This program is a recursive image randomizer. It shuffles the images it finds in a given folder by default using a config file listing every image it found.\nExample Usage:\n./wallscroll -path './folder-with-images' -shuffle -name 'collection_name'\n(shuffle is on by default)");
                std::process::exit(0); // exit without error
            }
            _ => i += 1,
        }
    }
    if arg_path == "" {
        eprintln!("Please provide a folder path using the '-path' argument.");
        std::process::exit(1); //exit with error
    }
    if arg_name == "" {
        eprintln!("Please provide a collection name for the images found using the '-name' argument.");
        std::process::exit(1); //exit with error
    }
    // Make folder with config (idk how to check if it exsists rn lol)
    let mut cnf_path = home_dir().unwrap();
    cnf_path.push(".config/wallscroll/"); // CONFIG file path is /home/user/.config/wallscroll/config
    fs::create_dir_all(cnf_path.clone()).unwrap();
    // println!("{}",format!("Created {:?} successfully.",&cnf_path).bright_green());
    cnf_path.push(arg_name);
    if !std::path::Path::new(&cnf_path).exists() { // If config no exsist, make
        File::create(&cnf_path).expect("Config file creation error. Do you have the right perms?");
        // println!("Created config.");
    } else {
        // println!("Reading config!")
    }


    // IF CONTENTS IN FILE then skip this:
    let mut config = File::open(&cnf_path).expect("Can't open config file!");
    let contents = BufReader::new(&config).lines();
    let mut images: Vec<String> = Vec::new();

    // dbg!(&contents);
    if contents.count() == 0 {
        // write mode
        config = File::create(&cnf_path).expect("Can't create config file!");
        // Parse image list into a vec!
        // Maybe convert to json and write to file?
        // Filter to be any type of image! FEH supports these so it will use them:
        // jpeg, png, pnm, tiff, bmp, gif (first frame only)
        images = WalkDir::new(&arg_path)
            .into_iter()
            .filter(|dir_entry| dir_entry.as_ref().unwrap().path().is_file())
            .filter(|e| (format!("{:?}",e.as_ref().unwrap()).contains(".jpeg")||format!("{:?}",e.as_ref().unwrap()).contains(".jpg")||format!("{:?}",e.as_ref().unwrap()).contains(".png")||format!("{:?}",e.as_ref().unwrap()).contains(".pnm")||format!("{:?}",e.as_ref().unwrap()).contains(".tiff")||format!("{:?}",e.as_ref().unwrap()).contains(".bmp")||format!("{:?}",e.as_ref().unwrap()).contains(".gif"))) 
            .map(|dir_entry| dir_entry.unwrap().path().to_str().unwrap().to_owned())
            .collect();
        // dbg!(&images); // now we need to write this to a config or file of sometype
        // when the vector is generated or read in, use this to choose and return
        for img in &images {
            // write image to file
            println!("{}",&img);
            writeln!(&mut config, "{}", &img).unwrap();
        }
    } 
    // println!("Config was created!");
    // READ entire file in, print first line, write rest back to file 
    let mut file = OpenOptions::new()
        .read(true)
        .open(&cnf_path)
        .expect("Error opening the config file!");
    println!(); // return the first line!
    let mut lines = BufReader::new(file).lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    dbg!(&lines);
    lines.shuffle(&mut thread_rng());
    println!("after shuff");
    dbg!(&lines);
    println!("{}",&lines.pop().expect("Error reading the line!"));
    let mut file = OpenOptions::new()
        .write(true)
        .open(&cnf_path)
        .expect("Error opening the config file!");
    for l in lines {
        println!("Writing {} ",&l);
        writeln!(&mut file, "{}",l).unwrap();
    }
}
