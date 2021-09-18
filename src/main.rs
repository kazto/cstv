use std::env;
use std::fs::File;
// use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};
use rand::prelude::*;


#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
enum Color {
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

fn count_max_columns(csv_data: Vec<String>) -> usize {
    let mut count: usize = 0;

    for line in csv_data {
        let line_elements: Vec<&str> = line.split(",").collect();
        if count < line_elements.len() {
            count = line_elements.len();
        }
    }
    return count;
}

#[test]
fn test_count_max_columns() {
    let input: Vec<String> = 
        vec!["foo,bar,buz", "hoge,hero,hoe,hoe,hoe", "xxx,yyyyy,zz,aaa"]
        .iter().map(|s| s.to_string()).collect();
    let count: usize = count_max_columns(input);
    assert_eq!(count, 5);
}

fn decide_colors(csv_data: Vec<String>) -> Vec<Color> {
    let colors: Vec<Color> = vec![
        Color::Red, Color::Green, Color::Blue,
        Color::Cyan, Color::Magenta, Color::Yellow,
        Color::DarkRed, Color::DarkGreen, Color::DarkBlue,
        Color::DarkCyan, Color::DarkMagenta, Color::DarkYellow
    ];
    let mut result: Vec<Color> = vec![];
    let max_size = count_max_columns(csv_data);
    let mut rng = rand::thread_rng();
    for _ in 0..max_size {
        result.push(colors[rng.gen_range(0..colors.len())])
    }

    return result;
}

fn colorize_cell(line: String) -> String {
    // この辺から再開
}

fn colorize(csv_data: Vec<String>, colors: Vec<Color>) -> Vec<String> {
    let mut colorized: Vec<String> = vec![];

    return colorized;
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

    let colors = decide_colors(csv_data);
    
    
    
    println!("Hello, world!");
}
