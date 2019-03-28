extern crate dirs;

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path;
use std::path::Path;

/// Create folder by path
/// Give a path and create a folder with sub folders
/// If the folder exists then it wont replace it!
/// TODO: use result! Refact!
pub fn create_folder_by_path(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Get home path
/// Work on windows, mac, linux
/// # Examplexo
/// ```rust
/// println!("Home path is: {}",get_home_path())
/// ```
pub fn get_home_path() -> Option<path::PathBuf> {
    dirs::home_dir()
}

/// Read file to string
/// Gets a File reference, tries to read it, once its done returns a Result
pub fn read_file_to_string(file: &mut File) -> Result<String, io::Error> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Write string to file
/// Gets a file reference and a string reference,
/// once its done, tries to write the string as byte to the file
/// Return Result.
pub fn write_string_to_file(path: &mut File, content: &String) -> std::io::Result<()> {
    // fs::write(path, content)
    path.write(content.as_bytes())?;
    Ok(())
}

/// Create file by path
/// Gets a Path reference, create a File and returns it as an IO Result
///
/// TODO: Maybe use File::create() directly, and remove this funtion?
pub fn create_file_from_path(file_path: &Path) -> io::Result<File> {
    File::create(file_path)
}

/// Get file names within a given path reference
/// 1. Check the given path is a directory
/// 2. Tries to read it
/// 3. Filter the content and collect all the file names into a vector
/// 4. Returns the 'names' vector
///
/// TODO: Concider using Result as return
/// TODO: Check the error handling. Expect? or using something more?
pub fn get_files_from_dir(path: &Path) -> Vec<String> {
    if path.is_dir() {
        let names = fs::read_dir(path)
            .expect("Error during reading folder..")
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| String::from(s)))
                })
            })
            .collect::<Vec<String>>();
        return names;
    }
    Vec::new()
}
