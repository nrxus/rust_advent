use solvers::*;

pub struct Answer {
    pub a: i32,
    pub b: i32,
}

pub struct ProblemBuilder;

impl ProblemBuilder {
    pub fn build(number: u32) -> Result<Problem, &'static str> {
        match number {
            1 => Ok(Problem::new(one::solve)),
            2 => Ok(Problem::new(two::solve)),
            3...25 => Err("Not yet implemented ☹"),
            _ => Err("This problem number does not exist!"),
        }
    }
}

pub struct Problem {
    solver: Box<Fn(&str) -> Answer>,
}

impl Problem {
    fn new<S: 'static>(solver: S) -> Problem
        where S: Fn(&str) -> Answer
    {
        Problem { solver: Box::new(solver) }
    }

    pub fn solve(&self, input: &str) -> Answer {
        (self.solver)(input)
    }
}
