use std::collections::HashSet;

use advent_problem::Answer;

pub fn solve(input: &str) -> Answer {
    Answer {
        a: num_houses(input, 1usize) as i32,
        b: num_houses(input, 2usize) as i32,
    }
}

fn num_houses(input: &str, num: usize) -> usize {
    if num == 0 {
        return 0;
    }

    let origin = House { x: 0, y: 0 };

    let mut santas = Vec::with_capacity(num);

    for _ in 0..num {
        santas.push(Santa::new(origin));
    }

    let mut houses = HashSet::new();
    houses.insert(origin);

    let mut iter = input.chars();
    'outer: loop {
        for santa in &mut santas {
            let c = iter.next();
            let c = match c {
                Some(v) => v,
                None => break 'outer,
            };

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
    fn new(house: House) -> Santa {
        Santa { house: house }
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
