use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;
use clap::{App, Arg};

fn main() {    
    let matches = App::new("aoc03b")
        .about("Advent of Code 2020 Day 3 Puzzle B solution")
        .arg(Arg::with_name("INPUT")
            .required(true)
            .index(1))
        .get_matches();

    let input_file_path = matches.value_of("INPUT").unwrap();

    if let Some(toboggan_map) = read_input_lines(input_file_path) {
        let first_route = punch_trees(&toboggan_map, (1, 1));

        let second_route = punch_trees(&toboggan_map, (1, 3));

        let third_route = punch_trees(&toboggan_map, (1, 5));

        let fourth_route = punch_trees(&toboggan_map, (1, 7));

        let fifth_route = punch_trees(&toboggan_map, (2, 1));

        let total_trees_multiplied = first_route * second_route * third_route * fourth_route * fifth_route;

        println!("Answer: {:?}", total_trees_multiplied)
    }
}

fn read_input_lines<P>(path: P) -> Option<Vec<Vec<bool>>>
where P: AsRef<Path>, {
    let input_file = File::open(path).ok()?; 
    let toboggan_map = BufReader::new(input_file).lines()
        .filter_map(Result::ok)
        .map(read_trees)
        .collect();

    Some(toboggan_map)
}

fn read_trees(line: String) -> Vec<bool> {
    let owo_what_s_this: Vec<bool> = line.chars().map(|x| x == '#').collect();

    // println!("{:?}", owo_what_s_this);

    owo_what_s_this
}

fn punch_trees(toboggan_map: &Vec<Vec<bool>>, direction: (usize, usize)) -> i64 {
    let bounds = (toboggan_map.len(), toboggan_map.first().unwrap().len());
    println!("bounds: {:?}", bounds);

    let coords = (1..bounds.0).map(|x| (x*direction.0, x*direction.1 % bounds.1)).filter(|(x, _y)| x < &bounds.0);
    println!("coords: {:?}", coords);

    let trees_hit = coords.fold(0, |x, y| x + if toboggan_map[y.0][y.1] {1} else {0});
    println!("Trees hit: {:?}", trees_hit);

    trees_hit
}