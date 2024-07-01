/**
 * Main file to call individual solutions to the Advent of Code 2023.
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
    
    let day_map = [15, 23];
    
    // Put all the solutions into an array
    let solutions = [
        solutions::day15::day15::solution,
        solutions::day23::day23::solution,
    ];
    
    assert_eq!(day_map.len(), solutions.len());
    
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args[1] == "all" {
        // Run all solutions
        
        for (day, sol) in solutions.iter().enumerate() {
            // Get input from file
            let file_name = format!("{input_path}/day{}_input.txt", day_map[day]);
            println!("{file_name}");
            let input = read_to_string(file_name)
                .unwrap()
                .lines()
                .map(String::from)
                .collect();
            
            println!("Day {}", day_map[day]);
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
        if !day_map.contains(&day) {
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
        solutions[day_map.iter().position(|d| d == &day).unwrap()](input);
        
    }
    
}


