use solvers::*;

pub struct Answer {
    pub a: i32,
    pub b: i32,
}

pub struct Problem {
    solver: Box<Fn(&str) -> Answer>,
}

impl Problem {
    pub fn new(number: u32) -> Result<Problem, &'static str> {
        let problem = match number {
            1 => Ok(Problem::problemify(one::solve)),
            2 => Ok(Problem::problemify(two::solve)),
            3...25 => Err("Not yet implemented ☹"),
            _ => Err("This problem number does not exist!"),
        };
        problem
    }

    pub fn solve(&self, input: &str) -> Answer {
        (self.solver)(input)
    }

    pub fn problemify<S: 'static>(solver: S) -> Problem
        where S: Fn(&str) -> Answer
    {
        Problem { solver: Box::new(solver) }
    }
}
