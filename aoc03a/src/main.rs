use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use clap::{App, Arg};

fn main() {    
    let matches = App::new("aoc03a")
        .about("Advent of Code 2020 Day 3 Puzzle A solution")
        .arg(Arg::with_name("INPUT")
            .required(true)
            .index(1))
        .get_matches();

    let input_file_path = matches.value_of("INPUT").unwrap();

    if let Some(toboggan_map) = read_input_lines(input_file_path) {
        let direction = (1, 3);
        let bounds = (toboggan_map.len(), toboggan_map.first().unwrap().len());
        println!("bounds: {:?}", bounds);

        let coords = (1..bounds.0).map(|x| (x*direction.0, x*direction.1 % bounds.1));
        println!("coords: {:?}", coords);

        let trees_hit = coords.fold(0, |x: i64, y| x + if toboggan_map[y.0][y.1] {1} else {0});

        println!("Trees hit: {:?}", trees_hit)
    }
}

fn read_input_lines<P>(path: P) -> Option<Vec<Vec<bool>>>
where P: AsRef<Path>, {
    let input_file = File::open(path).ok()?; 
    Some(BufReader::new(input_file).lines()
        .filter_map(Result::ok)
        .map(read_trees)
        .collect())
}

fn read_trees(line: String) -> Vec<bool> {
    let owo_what_s_this: Vec<bool> = line.chars().map(|x| x == '#').collect();

    owo_what_s_this
}