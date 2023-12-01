use std::fs;

pub fn file_reader<S: Into<String>>(folder: S) -> Vec<String> {
    let path_str = format!(
        "/Users/alielnour/Dev/rust/aoc23/src/{}/input.txt",
        folder.into()
    );
    
    fs::read_to_string(std::path::Path::new(&path_str))
        .expect("Error reading file.")
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}