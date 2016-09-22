use std::collections::HashSet;

use advent_problem::Answer;

pub fn solve(input: &str) -> Answer {
    Answer {
        a: num_houses(input) as i32,
        b: 0,
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct House {
    x: i32,
    y: i32,
}

impl House {
    fn move_to(&self, direction: char) -> House {
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

fn num_houses(input: &str) -> usize {
    let mut current = House { x: 0, y: 0 };
    let mut houses = HashSet::new();
    houses.insert(current);
    for c in input.chars() {
        current = current.move_to(c);
        houses.insert(current);
    }
    houses.len()
}
