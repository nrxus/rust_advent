pub fn solve(input: &str) {
    let count = count(&input);
    let position = find_basement(&input);
    println!("1A: {}", count);
    println!("1B: {}", position);
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
    let mut answer = 0;
    let mut count = 0;
    for c in input.chars() {
        answer += 1;
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
    answer
}
