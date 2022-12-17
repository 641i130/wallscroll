use std::fs;
use std::io;
use std::io::{Write,Read,Seek,BufReader,BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // specify the path to the file
    let file_path = Path::new("./test.txt");
    
    // open the file in read-write mode
    let rfile = fs::OpenOptions::new()
        .read(true)
        .write(false)
        .open(file_path)?;

    let wfile = fs::OpenOptions::new()
        .read(false)
        .write(true)
        .open(file_path)?;

    let mut reader = io::BufReader::new(rfile);
    let mut writer = io::BufWriter::new(wfile);

    // read the first line from the file
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    println!("{first_line}");

    // write the remaining lines to the file, skipping the first line
    let mut lines = String::new();
    reader.read_to_string(&mut lines)?;
    dbg!(&lines);
    writer.write_all(lines.as_bytes())?;

    Ok(())
}
