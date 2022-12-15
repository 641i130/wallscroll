//! This was written by ChatGPT, but I had to clean up some things.
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use rand::seq::SliceRandom;

const MAX_OPEN_FILES: usize = 1024; // maximum number of open files allowed by the system
const MAX_RECURSION_DEPTH: usize = 100; // maximum recursion depth allowed

fn main() -> io::Result<()> {
    // specify the directory to traverse
    let dir = Path::new("./testing/");

    // create a vector to store the names of image files
    let mut image_files = Vec::new();

    // recursively visit all files in the directory and its subdirectories
    traverse_dir(dir, 0, &mut image_files)?;

    // shuffle the list of image files
    let mut rng = rand::thread_rng();
    image_files.shuffle(&mut rng);

    // create a new config file to write the list of image files to
    let mut file = fs::File::create("image_files.config")?;

    // write the list of shuffled image files to the config file, one per line
    for img in image_files {
        file.write_all(img.as_bytes())?;
        file.write_all(b"\n")?;
    }

    Ok(())
}

fn traverse_dir(dir: &Path, depth: usize, image_files: &mut Vec<String>) -> io::Result<()> {
    // check if the maximum recursion depth has been reached
    if depth >= MAX_RECURSION_DEPTH {
        return Ok(());
    }

    // check if the maximum number of open files has been reached
    if image_files.len() >= MAX_OPEN_FILES {
        return Ok(());
    }

    // visit all files in the directory
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // if the path is a directory, recurse into it
        if path.is_dir() {
            traverse_dir(&path, depth + 1, image_files)?;
        } else {
            // if the path is an image file, add its name to the list
            if let Some(ext) = path.extension() {
                if ext == "png" || ext == "jpeg" || ext == "jpg" || ext == "bmp" {
                    image_files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(())
}
