use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use clap::{App, Arg};

fn main() {
    let matches = App::new("aoc01a")
        .about("Advent of Code 2020 Day 1 Puzzle 1 solution")
        .arg(Arg::with_name("INPUT")
            .required(true)
            .index(1))
        .get_matches();

    let input_file_path = matches.value_of("INPUT").unwrap();

    if let Some(input_lines) = read_input_lines(input_file_path) {
        let count = input_lines.iter().filter(is_valid_password).count() as i32;
    
        println!("There are {} valid passwords!", count)
    }
}

fn read_input_lines<P>(path: P) -> Option<Vec<(i32, i32, char, String)>>
where P: AsRef<Path>, {
    let input_file = File::open(path).ok()?; 
    BufReader::new(input_file).lines()
        .filter_map(Result::ok)
        .map(|line| split_input_line(line))
        .collect()
}

fn split_input_line(line: String) -> Option<(i32, i32, char, String)> {
    let split_line: Vec<&str> = line.split(|c: char| !c.is_alphanumeric()).collect();
    let split = (
        split_line[0].parse::<i32>().ok()?, 
        split_line[1].parse::<i32>().ok()?, 
        split_line[2].chars().next()?, 
        split_line[4].to_owned(),
    );
    Some(split)
}

fn is_valid_password((min, max, letter, password): &&(i32, i32, char, String)) -> bool {
    let number_of_matched_letters = password.matches(*letter).count() as i32;

    return *min <= number_of_matched_letters && number_of_matched_letters <= *max
}
