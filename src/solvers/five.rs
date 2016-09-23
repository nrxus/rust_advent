use advent_problem::Answer;

pub fn solve(input: &str) -> Answer {
    Answer {
        a: solution_a(input),
        b: -1,
    }
}

fn solution_a(input: &str) -> i32 {
    input.lines().map(|line| is_nice(line) as i32).sum()
}

fn is_nice(input: &str) -> bool {
    let mut vowel_count = 0;
    let mut found_pair = false;
    let mut last_letter = '\0';

    for c in input.chars() {
        if is_invalid(last_letter, c) {
            return false;
        }

        if vowel_count < 3 {
            vowel_count += is_vowel(c) as i32;
        }

        if !found_pair {
            found_pair = last_letter == c;
        }

        last_letter = c;
    }

    (vowel_count > 2) && found_pair
}

fn is_vowel(letter: char) -> bool {
    letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u'
}

fn is_invalid(previous: char, current: char) -> bool {
    let mut pair = previous.to_string();
    pair.push(current);
    pair == "ab" || pair == "cd" || pair == "pq" || pair == "xy"
}