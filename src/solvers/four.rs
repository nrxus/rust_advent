use advent_problem::Answer;

extern crate crypto;
use self::crypto::md5::Md5;
use self::crypto::digest::Digest;

pub fn solve(input: &str) -> Answer {
    Answer {
        a: solution_a(input),
        b: -1,
    }
}

fn solution_a(input: &str) -> i32 {
    let mut md5 = Md5::new();

    for i in 1.. {
        let secret = input.trim().to_owned() + &i.to_string();
        md5.input(secret.as_bytes());
        let mut hash = [0; 16];
        md5.result(&mut hash);
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return i;
        }
        md5.reset();
    };

    -1
}
