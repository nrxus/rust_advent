use std::io::{self, Read};
use std::env;

fn count(input: &str) -> i32 {
    let mut answer = 0;
    for c in input.chars() {
        answer = if c == '(' {
            answer + 1
        } else if c == ')' {
            answer - 1
        } else {
            answer
        }
    }
    answer
}

fn one() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let answer = count(&input);
    println!("{}", answer);
}

fn main() {
    if env::args().len() != 2 {
        panic!("Incorrect number of parameters!");
    }
    
    let problem_number = env::args().nth(1).unwrap();
    let problem_number: u32 =
        problem_number.trim().parse().expect("Please type a number for the problem (i.e., 1)!");
    match problem_number {
        1 => one(),
        2...25 => println!("Not yet implemented ☹"),
        _ => println!("This problem number does not exist!"),
    }
    return;
}
