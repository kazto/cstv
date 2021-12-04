use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, stdout, Write, BufWriter};
use rand::prelude::*;
use std::collections::HashMap;
use std::clone::Clone;

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
    DarkYellow,
    Reset
}

fn initialize_color_maps() -> HashMap<Color, String> {
    let keys = vec![
        Color::Red, Color::Green, Color::Blue,
        Color::Cyan, Color::Magenta, Color::Yellow,
        Color::DarkRed, Color::DarkGreen, Color::DarkBlue,
        Color::DarkCyan, Color::DarkMagenta, Color::DarkYellow,
        Color::Reset
    ];
    let values = vec![
        String::from("\x1b[31m"), String::from("\x1b[32m"), String::from("\x1b[34m"), 
        String::from("\x1b[36m"), String::from("\x1b[35m"), String::from("\x1b[33m"), 
        String::from("\x1b[31;1m"), String::from("\x1b[32;1m"), String::from("\x1b[34;1m"), 
        String::from("\x1b[36;1m"), String::from("\x1b[35;1m"), String::from("\x1b[33;1m"), 
        String::from("\x1b[0m"), 
    ];
    let mut color_maps: HashMap<Color, String> = HashMap::new();
    keys.iter().zip(values.iter()).for_each(|(&k, v)| {
        color_maps.insert(k, String::from(v));
        return ().into();
    });
    return color_maps;
}

#[test]
fn test_initialize_color_maps() {
    let color_maps = initialize_color_maps();
    assert_eq!(color_maps[&Color::Red], 0xff0000i32);
}

fn count_max_columns(csv_data: &Vec<String>) -> usize {
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
    let count: usize = count_max_columns(&input);
    assert_eq!(count, 5);
}

fn decide_colors(csv_data: &Vec<String>, color_maps: HashMap<Color, String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let max_size = count_max_columns(csv_data);
    let mut rng = rand::thread_rng();
    let mut keys = color_maps.keys().collect::<Vec<&Color>>();
    keys.shuffle(&mut rng);
    for _ in 0..max_size {
        let random_key = keys.pop().unwrap();
        result.push(String::from(&color_maps[random_key]));
    }

    return result;
}

fn colorize_cell(cell: &str, color: &str) -> String {
    return [color, cell, "\x1b[0m"].concat();
}

fn colorize(csv_data: &Vec<String>, colors: Vec<String>) -> Vec<String> {
    return csv_data.iter().map(|line|
        line.split(",")
            .collect::<Vec<&str>>()
            .iter()
            .zip(colors.iter())
            .map(|(cell, color)|
                colorize_cell(cell, color)
            )
            .collect::<Vec<String>>()
            .join(",")
    )
    .collect::<Vec<String>>();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let csv_file = &args[1];
    let fh = File::open(csv_file).expect("file not found!");
    let reader = BufReader::new(fh);
    let lines = reader.lines();
    let mut csv_data: Vec<String> = vec![];
    let color_maps: HashMap<Color, String>;

    color_maps = initialize_color_maps();

    for line in lines {
        match line {
            Err(_) => (),
            Ok(v) => csv_data.push(v)
        }
    }

    let colors = decide_colors(&csv_data, color_maps);
    let result = colorize(&csv_data, colors);

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    for line in result {
        out.write(line.as_bytes()).unwrap();
        out.write(b"\n").unwrap();
    }
}

#[test]
fn test_colorize_cell() {
    println!("\x1b[31mred\x1b[0mblack\x1b[36mcyan\x1b[0m");
}
