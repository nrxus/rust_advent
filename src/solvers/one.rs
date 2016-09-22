use advent_problem::Answer;

pub fn solve(input: &str) -> Answer {
    let count = count(input);
    let position = find_basement(input);
    Answer {
        a: count,
        b: position,
    }
}

fn count(input: &str) -> i32 {
    let mut count = 0;
    for c in input.chars() {
        count = if c == '(' {
            count + 1
        } else if c == ')' {
            count - 1
        } else {
            count
        }
    }
    count
}

fn find_basement(input: &str) -> i32 {
    let mut position = 0;
    let mut count = 0;
    for c in input.chars() {
        position += 1;
        count = if c == '(' {
            count + 1
        } else if c == ')' {
            count - 1
        } else {
            count
        };
        if count == -1 {
            break;
        }
    }
    position
}
