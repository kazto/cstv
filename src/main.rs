use std::env;
use std::fs::File;
// use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use rand::prelude::*;
use termion::color;
use std::collections::HashMap;


#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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

#[derive(Hash, Eq, PartialEq, Debug)]
struct ColorMap {
    color: Color,
    code: i32
}

impl ColorMap {
    fn new(color: &Color, code: i32) -> ColorMap {
        ColorMap { color: *color, code: code}
    }
}


fn initializeColorMaps() -> &HashMap<Color, i32> {
    let mut colorMaps: HashMap<_, _> = HashMap::new();
    let keys = vec![
        Color::Red, Color::Green, Color::Blue,
        Color::Cyan, Color::Magenta, Color::Yellow,
        Color::DarkRed, Color::DarkGreen, Color::DarkBlue,
        Color::DarkCyan, Color::DarkMagenta, Color::DarkYellow
    ];
    let values = vec![
        0xff0000i32, 0x00ff00i32, 0x0000ffi32,
        0x00ffffi32, 0xff00ffi32, 0xffff00i32,
        0x7f0000i32, 0x007f00i32, 0x00007fi32,
        0x007f7fi32, 0x7f007fi32, 0x7f7f00i32
    ];
    let color_maps: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

    return color_maps.clone();
}

#[test]
fn test_initializeColorMaps() {
    initializeColorMaps();
    assert_eq!(colorMaps[&Color::Red].code, 0xff0000i32);
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
    return line.to_string();
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

    initializeColorMaps();

    for line in lines {
        csv_data.push(line.unwrap())
    }

    let colors = decide_colors(csv_data);
    
    
    
    println!("Hello, world!");
}
