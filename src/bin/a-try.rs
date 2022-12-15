//! This was written by ChatGPT, but I had to clean up some things.
use std::fs;
use std::io::Write;
use std::io;
use std::path::Path;
use rand::seq::SliceRandom;

fn main() -> io::Result<()> {
    // specify the directory to traverse
    let dir = Path::new("./testing/");

    // create a vector to store the names of image files
    let mut image_files = Vec::new();

    // recursively visit all files in the directory and its subdirectories
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // if the path is a directory, recurse into it
        if path.is_dir() {
            main()?;
        } else {
            // if the path is an image file, add its name to the list
            if let Some(ext) = path.extension() {
                if ext == "png" || ext == "jpeg" || ext == "jpg" || ext == "bmp" {
                    image_files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

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
