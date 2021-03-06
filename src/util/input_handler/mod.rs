use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_file (folder: &str) -> Vec<String> {
    let path = format!("src/puzzles/{}/input.txt", folder);

    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.expect("failed to read line"));
    }

    return lines; 
}