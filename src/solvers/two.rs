use advent_problem::Answer;

struct BoxDims {
    length: i32,
    width: i32,
    height: i32,
}

impl BoxDims {
    fn paper_area(&self) -> i32 {
        self.surface_area() + self.side_areas().iter().min().unwrap()
    }

    fn ribbon_length(&self) -> i32 {
        self.side_perimeters().iter().min().unwrap() + self.volume()
    }

    fn surface_area(&self) -> i32 {
        self.side_areas().iter().map(|area| 2 * area).sum()
    }

    fn volume(&self) -> i32 {
        self.width * self.height * self.length
    }

    fn side_areas(&self) -> [i32; 3] {
        [self.length * self.width,
         self.width * self.height,
         self.height * self.length]
    }

    fn side_perimeters(&self) -> [i32; 3] {
        [2 * (self.length + self.width),
         2 * (self.width + self.height),
         2 * (self.height + self.length)]
    }
}

pub fn solve(input: &str) -> Answer {
    let boxes = parse(input);
    Answer {
        a: paper_for_boxes(&boxes),
        b: ribbon_for_boxes(&boxes),
    }
}

fn parse(input: &str) -> Vec<BoxDims> {
    input.lines()
        .map(|line| {
            let v: Vec<&str> = line.split(|c| c == 'X' || c == 'x')
                .collect();

            BoxDims {
                length: v[0].parse().unwrap(),
                width: v[1].parse().unwrap(),
                height: v[2].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

fn paper_for_boxes(boxes: &[BoxDims]) -> i32 {
    boxes.iter().map(|dims| dims.paper_area()).sum()
}

fn ribbon_for_boxes(boxes: &[BoxDims]) -> i32 {
    boxes.iter().map(|dims| dims.ribbon_length()).sum()
}
