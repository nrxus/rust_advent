use std::collections::HashSet;

use advent_problem::Answer;

pub fn solve(input: &str) -> Answer {
    Answer {
        a: num_houses(input, 1usize) as i32,
        b: num_houses(input, 2usize) as i32,
    }
}

fn num_houses(input: &str, num: usize) -> usize {
    let mut santas = Vec::with_capacity(num);

    for _ in 0..num {
        santas.push(Santa::new());
    }

    houses_visited(input, &mut santas).len()
}

fn houses_visited(input: &str, santas: &mut Vec<Santa>) -> HashSet<House> {
    let mut houses = HashSet::new();
    for santa in &*santas {
        houses.insert(santa.house);
    }

    let mut iter = input.chars();
    'outer: loop {
        for santa in &mut *santas {
            let c = iter.next();
            let c = match c {
                Some(v) => v,
                None => break 'outer,
            };

            santa.move_by(c);
            houses.insert(santa.house);
        }
    }

    houses
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
