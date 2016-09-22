use advent_problem::Answer;

macro_rules! min {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        {
            use std::cmp::min;
            min($x, min!( $($xs),+ ))
        }
    };
}

macro_rules! max {
    ($x:expr) => ( $x );
    ($x:expr, $($xs:expr),+) => {
        {
            use std::cmp::max;
            max($x, max!( $($xs),+ ))
        }
    };
}

struct BoxDims {
    length: i32,
    width: i32,
    height: i32,
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
            let v: Vec<&str> = line.split(|c| c == 'X' || c == 'x').collect();
            BoxDims {
                length: v[0].parse().unwrap(),
                width: v[1].parse().unwrap(),
                height: v[2].parse().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

fn paper_for_boxes(boxes: &[BoxDims]) -> i32 {
    boxes.iter().fold(0, |acc, x| acc + paper_for_box(x))
}

fn ribbon_for_boxes(boxes: &[BoxDims]) -> i32 {
    boxes.iter().fold(0, |acc, x| acc + ribbon_for_box(x))
}

fn paper_for_box(dims: &BoxDims) -> i32 {
    let side_area = dims.width * dims.height;
    let bottom_area = dims.length * dims.width;
    let front_area = dims.length * dims.height;

    2 * side_area + 2 * bottom_area + 2 * front_area + min!(side_area, bottom_area, front_area)
}

fn ribbon_for_box(dims: &BoxDims) -> i32 {
    let distance =
        2 * (dims.width + dims.height + dims.length - max!(dims.width, dims.height, dims.length));
    distance + (dims.width * dims.height * dims.length)
}
