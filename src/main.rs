use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

enum Color {
    Default,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    DarkRed,
    DarkGreen,
    DarkBlue,
    DarkCyan,
    DarkMagenta,
    DarkYellow
}

fn decide_colors(csv_data: Vec<String>) -> Vec<Color> {
    let mut colors: Vec<Color> = vec![];
    // この辺から再開
    return colors;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let csv_file = &args[1];
    let fh = File::open(csv_file).expect("file not found!");
    let reader = BufReader::new(fh);
    let lines = reader.lines();
    let mut csv_data: Vec<String> = vec![];

    for line in lines {
        csv_data.push(line.unwrap())
    }
    
    
    
    println!("Hello, world!");
}
