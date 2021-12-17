use crate::solutions::utils;
use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list0,
    sequence::separated_pair, IResult,
};
use std::cmp;
use std::fs;

static INPUT_FILE: &str = "./inputs/day5/input.txt";

pub fn solve() {
    let inputs = read_input(INPUT_FILE);
    let result = &grid(1000, 1000, &inputs);
    print_grid(result);
    let mut count = 0;
    for row in result {
        for point in row {
            if *point > 1 {
                count += 1;
            }
        }
    }

    println!("{}", count);
    println!("WIP");
}

fn read_input(path: &str) -> Vec<Line> {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    let (_, lines) = separated_list0(newline, Line::parse)(&contents).unwrap();
    lines
}

type Point = (usize, usize);

fn parse_point(input: &str) -> IResult<&str, Point> {
    separated_pair(utils::parse_usize, tag(","), utils::parse_usize)(input)
}

fn grid(heigth: usize, width: usize, lines: &Vec<Line>) -> Vec<Vec<usize>> {
    let mut grid = vec![vec![0; width]; heigth];

    let is_diagonal = |p1: Point, p2: Point| -> bool {
        (cmp::max(p1.0, p2.0) - cmp::min(p1.0, p2.0))
            == (cmp::max(p1.1, p2.1) - cmp::min(p1.1, p2.1))
    };

    for line in lines {
        let (x1, y1) = line.start;
        let (x2, y2) = line.end;
        if line.is_horizontal() {
            for i in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                grid[y1][i] += 1;
            }
        } else if line.is_vertical() {
            for i in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                grid[i][x1] += 1;
            }
        } else if is_diagonal(line.start, line.end) {
            println!("{:?} -> {:?}", line.start, line.end);
            let n_points = cmp::max(x1, x2) - cmp::min(x1, x2);
            println!("{}", n_points);
            for p in 0..=n_points {
                let i = if y2 >= y1 { y1 + p } else { y1 - p };
                let j = if x2 >= x1 { x1 + p } else { x1 - p };
                println!("{}, {}", j, i);
                grid[i][j] += 1;
            }
        }
    }

    grid
}

fn print_grid(grid: &Vec<Vec<usize>>) {
    for row in grid {
        for point in row {
            match point {
                0 => print!("."),
                other => print!("{}", other),
            }
        }
        println!("");
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_tuple((start, end): (Point, Point)) -> Self {
        Self { start, end }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let arrow = tag(" -> ");

        map(
            separated_pair(parse_point, arrow, parse_point),
            Line::from_tuple,
        )(input)
    }

    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_FILE: &str = "./inputs/day5/test.txt";
}
