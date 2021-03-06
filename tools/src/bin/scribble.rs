use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;

const SONNETS: &str = include_str!("../../data/sonnets.txt");
const TEMPLATE: &str = include_str!("../../data/template.html");

fn main() {
    let filename = match std::env::args().nth(1) {
        Some(s) => s,
        None => {
            eprintln!("must specify file");
            std::process::exit(1);
        }
    };

    let tlen = TEMPLATE.lines().count();
    let len = count_lines(&filename);

    match SONNETS.lines().nth(len - tlen) {
        Some(line) => append(&filename, line),
        None => truncate(&filename),
    }
}

fn count_lines(filename: &str) -> usize {
    let file = File::open(filename).expect("failed to open file");
    let reader = BufReader::new(file);
    reader.lines().count()
}

fn truncate(filename: &str) {
    OpenOptions::new()
        .write(true)
        .open(filename)
        .expect("failed to open file for truncate")
        .set_len(TEMPLATE.len() as u64)
        .expect("failed to truncate file");
}

fn append(filename: &str, line: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("failed to open file for append");
    file.write(line.as_bytes()).unwrap();
    file.write(b"<br>\n").unwrap();
}
