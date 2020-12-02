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

fn read_input_lines<P>(path: P) -> Option<Vec<(usize, usize, char, String)>>
where P: AsRef<Path>, {
    let input_file = File::open(path).ok()?; 
    BufReader::new(input_file).lines()
        .filter_map(Result::ok)
        .map(|line| split_input_line(line))
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
    if let (Some(first_letter), Some(second_letter)) = (password.chars().nth(*min - 1), password.chars().nth(*max - 1)) {
        let is_valid = (first_letter == *letter) ^ (second_letter == *letter);
        println!("First letter: {}, Second letter: {}, Letter to check: {}, Is valid: {}", first_letter, second_letter, *letter, is_valid);
        is_valid
    } else {
        false
    }
}
