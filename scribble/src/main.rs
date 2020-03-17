use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

const SONNETS: &str = include_str!("../data/sonnets.txt");

fn main() {
    // how many lines currently?
    let file = File::open("out.txt").unwrap();
    let reader = BufReader::new(file);
    let len = reader.lines().count();

    // now open for append and add next file
    if len == SONNETS.lines().count() {
        // all lines printed, truncate
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("out.txt")
            .unwrap();
    } else {
        // append next line
        let mut file = OpenOptions::new().append(true).open("out.txt").unwrap();
        let line = SONNETS.lines().skip(len).next().unwrap();
        file.write(line.as_bytes()).unwrap();
        file.write(b"\n").unwrap();
    }
}
