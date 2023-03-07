use std::fs::{OpenOptions};
use std::io::{Read, BufRead, BufReader, Result, Write};
use std::path::Path;

fn main() -> Result<()> {
    // specify the path to the file
    let file_path = Path::new("./test.txt");

    // open the file in read-write mode
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    // read the first line from the file
    let mut reader = BufReader::new(&file);
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    println!("{first_line}");
    
    let lines = reader.lines().skip(1)
        .map(|x| x.unwrap())
        .collect::<Vec<String>>().join("\n");
    std::fs::write("./test.txt", lines).expect("Can't write");
    //file.write_all(lines.as_bytes())?;

    Ok(())
}

