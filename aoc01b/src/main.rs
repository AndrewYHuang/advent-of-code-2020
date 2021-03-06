use std::io::{self, Lines, BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use clap::{App, Arg};

const SUM_NUMBER: i32 = 2020;

fn main() {
    let matches = App::new("aoc01b")
        .about("Advent of Code 2020 Day 1 Puzzle B solution")
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

        if let Some((a, b, c)) = find_outer_recursive(numbers) {
            println!("a = {}, b = {}, c = {}", a, b, c);

            let answer = a*b*c;

            println!("answer = {}", answer);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn find_outer_recursive(numbers: &mut Vec<i32>) -> Option<(i32, i32, i32)> {
    let current_number = numbers.pop()?;
    
    let clone_numbers = &mut numbers.clone();
    let target_number = SUM_NUMBER - current_number;

    if let Some((a, b)) = find_recursive(clone_numbers, target_number) {
        Some((current_number, a, b))
    } else {
        find_outer_recursive(numbers)
    }
}

fn find_recursive(numbers: &mut Vec<i32>, sum_number: i32) -> Option<(i32, i32)> {
    let current_number = numbers.pop()?;

    let target_number = sum_number - current_number;

    if numbers.contains(&target_number) {
        Some((current_number, target_number))
    } else {
        find_recursive(numbers, sum_number)
    }
}