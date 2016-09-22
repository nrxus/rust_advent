use std::collections::HashSet;

use advent_problem::Answer;

pub fn solve(input: &str) -> Answer {
    Answer {
        a: num_houses(input) as i32,
        b: 0,
    }
}

fn num_houses(input: &str) -> usize {
    let mut houses = HashSet::new();
    let santa = Santa::new();
    houses.insert(santa.house);

    {
        let mut santa = santa;
        for c in input.chars() {
            santa.move_by(c);
            houses.insert(santa.house);
        }
    }

    houses.len()
}

struct Santa {
    house: House,
}

impl Santa {
    fn new() -> Santa {
        Santa { house: House { x: 0, y: 0 } }
    }

    fn move_by(&mut self, direction: char) {
        self.house = self.house.next(direction);
    }
}


#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct House {
    x: i32,
    y: i32,
}

impl House {
    fn next(&self, direction: char) -> House {
        let (x, y) = match direction {
            '>' => (self.x + 1, self.y),
            '<' => (self.x - 1, self.y),
            '^' => (self.x, self.y + 1),
            'v' => (self.x, self.y - 1),
            _ => (self.x, self.y),
        };
        House { x: x, y: y }
    }
}
