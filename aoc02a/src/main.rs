use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use clap::{App, Arg};

fn main() {
    let matches = App::new("aoc02a")
        .about("Advent of Code 2020 Day 2 Puzzle A solution")
        .arg(Arg::with_name("INPUT")
            .required(true)
            .index(1))
        .get_matches();

    let input_file_path = matches.value_of("INPUT").unwrap();

    if let Some(input_lines) = read_input_lines(input_file_path) {
        let count = input_lines.iter().filter(is_valid_password).count();
    
        println!("There are {} valid passwords!", count)
    }
}

fn read_input_lines<P>(path: P) -> Option<Vec<(usize, usize, char, String)>>
where P: AsRef<Path>, {
    let input_file = File::open(path).ok()?; 
    BufReader::new(input_file).lines()
        .filter_map(Result::ok)
        .map(split_input_line)
        .collect()
}

fn split_input_line(line: String) -> Option<(usize, usize, char, String)> {
    let split_line: Vec<&str> = line.split(|c: char| !c.is_alphanumeric()).collect();
    let split = (
        split_line[0].parse::<usize>().ok()?, 
        split_line[1].parse::<usize>().ok()?, 
        split_line[2].chars().next()?, 
        split_line[4].to_owned(),
    );
    Some(split)
}

fn is_valid_password((min, max, letter, password): &&(usize, usize, char, String)) -> bool {
    let number_of_matched_letters = password.matches(*letter).count();

    return *min <= number_of_matched_letters && number_of_matched_letters <= *max
}
