use std::io::{self, Lines, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use clap::{App, Arg};

const SUM_NUMBER: i32 = 2020;

fn main() {
    let matches = App::new("aoc01a")
        .about("Advent of Code 2020 Day 1 Puzzle 1 solution")
        .arg(Arg::with_name("INPUT")
            .required(true)
            .index(1))
        .get_matches();

    let input_file_path = matches.value_of("INPUT").unwrap();

    if let Ok(lines) = read_lines(input_file_path) {
        let numbers: &mut Vec<i32> = &mut lines
            .filter_map(Result::ok)
            .map(|line: String| line.parse::<i32>())
            .filter_map(Result::ok)
            .collect();

        if let Some((a, b)) = find_recursive(numbers) {
            println!("a = {}, b = {}", a, b);

            let answer = a*b;

            println!("answer = {}", answer);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn find_recursive(numbers: &mut Vec<i32>) -> Option<(i32, i32)> {
    let current_number = numbers.pop()?;

    let target_number = SUM_NUMBER - current_number;

    if numbers.contains(&target_number) {
        Some((current_number, target_number))
    } else {
        find_recursive(numbers)
    }
}