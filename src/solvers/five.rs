use advent_problem::Answer;

pub fn solve(input: &str) -> Answer {
    Answer {
        a: solution(input, is_nice_a),
        b: solution(input, is_nice_b),
    }
}

fn solution<F: Fn(&str) -> bool>(input: &str, is_nice: F) -> i32 {
    input.lines().map(|line| is_nice(line) as i32).sum()
}

fn is_nice_a(input: &str) -> bool {
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

fn is_nice_b(input: &str) -> bool {
    let mut chars = input.chars();
    let mut last_letters = (chars.next().unwrap(), chars.next().unwrap());

    let mut pairs_found = (last_letters.0 == last_letters.1) as i32;
    let mut last_pair_letter = if pairs_found > 0 {
        '\0'
    } else {
        last_letters.1
    };
    let mut found_repeat = false;

    for c in chars {
        if !found_repeat {
            found_repeat = c == last_letters.0
        }
        if pairs_found < 2 {
            if c == last_pair_letter {
                pairs_found += 1;
                last_pair_letter = '\0';
            } else {
                last_pair_letter = c;
            }
        }
        last_letters.0 = last_letters.1;
        last_letters.1 = c;
    }

    pairs_found > 1 && found_repeat
}

fn is_vowel(letter: char) -> bool {
    letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u'
}

fn is_invalid(previous: char, current: char) -> bool {
    let mut pair = previous.to_string();
    pair.push(current);
    pair == "ab" || pair == "cd" || pair == "pq" || pair == "xy"
}
