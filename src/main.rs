use std::env;
use std::io::{self, Read};

extern crate advent_of_code;
use advent_of_code::advent_problem::ProblemBuilder;

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of parameters!");
    }

    let problem_number = env::args().nth(1).unwrap();
    let problem_number: u32 =
        problem_number.trim().parse().expect("Please type a number for the problem (i.e., 1)!");

    let problem = ProblemBuilder::build(problem_number);
    let problem = match problem {
        Ok(p) => p,
        Err(e) => panic!(e),
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let answer = problem.solve(&input);

    println!("solution to problem {}A: {}", problem_number, answer.a);
    println!("solution to problem {}B: {}", problem_number, answer.b);
}
