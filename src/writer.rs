use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Writes the provided content to a file at the specified filepath.
/// 
/// # Arguments
/// 
/// * `filepath` - A string slice that holds the path to the file.
/// * `content` - A `String` containing the content to be written to the file.
/// 
/// # Panics
/// 
/// This function will panic if the file cannot be created or if there is an error writing to it.
pub fn hrw_write_file(filepath: &str, content: String) {
    let path = Path::new(filepath);
    let display = path.display();
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {:?}", display, why),
        Ok(file) => file,
    };

    if let Err(why) = file.write_all(content.as_bytes()) {
        panic!("couldn't write to {}: {:?}", display, why);
    }
}
