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

struct Grid<T> {
    vec: Vec<Vec<T>>,
    width: usize,
    height: usize
}

impl<T> Grid<T> {
    pub fn from(vec: Vec<Vec<T>>) -> Self {
        let width = match vec.first() {
            Some(s) => s.len(),
            None => 0
        };
        let height = vec.len();
        Self {
            vec,
            width,
            height
        }
    }

    /* pub fn from_string_vec(vec: Vec<String>) -> Self {
        let vec = vec
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self::from(vec)
    } */
}