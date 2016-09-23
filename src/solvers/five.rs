use std::collections::HashSet;
use advent_problem::Answer;

pub fn solve(input: &str) -> Answer {
    Answer {
        a: solution_a("aeious"),
        b: -1,
    }
}

fn solution_a(input: &str) -> i32 {
    let mut answer = 0;
    for line in input.lines() {
        answer = answer + is_nice(line) as i32;
    }
    answer
}

fn is_nice(input: &str) -> bool {
    let mut vowel_count = 0;
    let mut vowels = HashSet::new();
    vowels.insert('a');
    vowels.insert('e');
    vowels.insert('i');
    vowels.insert('o');
    vowels.insert('u');
    let mut last_letter = input.chars().next().unwrap();

    if input.contains("ab") || input.contains("cd") || input.contains("pq") ||
       input.contains("xy") {
        return false;
    }

    for c in input.chars() {
        vowel_count = vowel_count + vowels.contains(&c) as i32;
    }

    vowel_count > 2
}
