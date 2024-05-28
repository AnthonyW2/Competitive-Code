/**
 * Main file to call individual solutions to the Advent of MAPS 2024.
 */

pub mod solutions;

// To run:
// $ cargo run -- <day> <input file>
// $ echo <input> | cargo run -- <day>
// $ cargo run -- all

use std::env;
use std::io::{self, BufRead};
use std::fs::read_to_string;

// To check for piped input
extern crate atty;

fn main() {
    
    let input_path = "./inputs";
    
    // Put all the solutions into an array
    let solutions = [
        solutions::day1::day1::solution,
        solutions::day2::day2::solution,
        solutions::day3::day3::solution,
        solutions::day4::day4::solution,
        solutions::day5::day5::solution,
        solutions::day6::day6::solution,
        solutions::day7::day7::solution,
        solutions::day8::day8::solution,
        solutions::day9::day9::solution,
        solutions::day10::day10::solution,
        solutions::day11::day11::solution,
        solutions::day12::day12::solution,
        solutions::day13::day13::solution,
        solutions::day14::day14::solution,
        solutions::day15::day15::solution,
        solutions::day16::day16::solution,
        solutions::day17::day17::solution,
        solutions::day18::day18::solution,
        solutions::day19::day19::solution,
        solutions::day20::day20::solution,
        solutions::day21::day21::solution,
        solutions::day22::day22::solution,
        solutions::day23::day23::solution,
        solutions::day24::day24::solution,
        solutions::day25::day25::solution,
        solutions::day26::day26::solution,
        solutions::day27::day27::solution,
        solutions::day28::day28::solution,
    ];
    
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args[1] == "all" {
        // Run all solutions
        
        for (day, sol) in solutions.iter().enumerate() {
            // Get input from file
            let file_name = format!("{input_path}/day{}_input.txt", day+1);
            let input = read_to_string(file_name)
                .unwrap()
                .lines()
                .map(String::from)
                .collect();
            
            println!("Day {}", day+1);
            sol(input);
            println!();
        }
        
    } else {
        // Run just one solution
        
        // Parse the day
        let day = args[1].parse::<usize>();
        if day.is_err() {
            println!("First argument is not \"all\" or a day number.");
            return;
        }
        let day = day.unwrap();
        if day < 1 || day > solutions.len() {
            println!("\"{}\" is not a valid day number", day);
            return;
        }
        
        // Get the necessary input
        let input: Vec<String>;
        if !atty::is(atty::Stream::Stdin) {
            // Input present on stdin
            let stdin = io::stdin();
            input = stdin.lock().lines().map(|l| l.unwrap()).collect();
            
        } else {
            // Get input from file
            let file_name;
            if args.len() >= 3 {
                file_name = format!("{}", args[2]);
            } else {
                file_name = format!("{input_path}/day{day}_input.txt");
            }
            input = read_to_string(file_name)
                .unwrap()
                .lines()
                .map(String::from)
                .collect();
        }
        
        // Execute the solution
        solutions[day-1](input);
        
    }
    
}


