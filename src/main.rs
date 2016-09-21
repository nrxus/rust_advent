use std::env;
use std::io::{self, Read};

extern crate advent_of_code;
use advent_of_code::*;

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of parameters!");
    }

    let problem_number = env::args().nth(1).unwrap();
    let problem_number: u32 =
        problem_number.trim().parse().expect("Please type a number for the problem (i.e., 1)!");

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let (a,b) = match problem_number {
        1 => one::solve(&input),
        2...25 => panic!("Not yet implemented ☹"),
        _ => panic!("This problem number does not exist!"),
    };

    println!("solution to problem {}A: {}", problem_number, a);
    println!("solution to problem {}B: {}", problem_number, b);
}
