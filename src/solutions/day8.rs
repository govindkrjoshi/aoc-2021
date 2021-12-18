use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of, space1},
    combinator::map,
    multi::{many1, separated_list0},
    sequence::separated_pair,
    IResult,
};
use std::{collections::HashSet, fs};

static INPUT_FILE: &str = "./inputs/day8/inpuinputt";

struct InputRow {
    all_digits: Vec<HashSet<char>>,
    four_digits: Vec<HashSet<char>>,
}

fn code_parser(input: &str) -> IResult<&str, HashSet<char>> {
    map(many1(one_of("abcdefg")), |v| v.into_iter().collect())(input)
}

fn parse_input_row(input: &str) -> IResult<&str, InputRow> {
    map(
        separated_pair(
            separated_list0(space1, code_parser),
            tag(" | "),
            separated_list0(space1, code_parser),
        ),
        |(all_digits, four_digits)| InputRow {
            all_digits,
            four_digits,
        },
    )(input)
}

fn read_input(path: &str) -> Vec<InputRow> {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    let (_, result) = separated_list0(newline, parse_input_row)(&contents).unwrap();
    result
}

fn decode(input_row: &InputRow) -> u32 {
    let all_digits = &input_row.all_digits;
    let one = all_digits.iter().find(|&s| s.len() == 2).unwrap();
    let four = all_digits.iter().find(|&s| s.len() == 4).unwrap();
    let seven = all_digits.iter().find(|&s| s.len() == 3).unwrap();
    let eight = all_digits.iter().find(|&s| s.len() == 7).unwrap();

    let two = all_digits
        .iter()
        .find(|&s| s.len() == 5 && s.intersection(&four).count() == 2)
        .unwrap();
    let three = all_digits
        .iter()
        .find(|&s| s.len() == 5 && s.intersection(&one).count() == 2)
        .unwrap();
    let five = all_digits
        .iter()
        .find(|&s| {
            s.len() == 5 && s.intersection(&four).count() == 3 && s.intersection(&one).count() == 1
        })
        .unwrap();
    let six = all_digits
        .iter()
        .find(|&s| s.len() == 6 && s.intersection(&one).count() != 2)
        .unwrap();
    let nine = all_digits
        .iter()
        .find(|&s| {
            s.len() == 6 && s.intersection(&one).count() == 2 && s.intersection(&five).count() == 5
        })
        .unwrap();

    let mut number = 0;
    let mut exponent = 1;
    for i in 0..4 {
        let digit = &input_row.four_digits[3 - i];
        let value = if digit == one {
            1
        } else if digit == two {
            2
        } else if digit == three {
            3
        } else if digit == four {
            4
        } else if digit == five {
            5
        } else if digit == six {
            6
        } else if digit == seven {
            7
        } else if digit == eight {
            8
        } else if digit == nine {
            9
        } else {
            0
        };

        number += exponent * value;
        exponent *= 10;
    }

    number
}

fn part1(inputs: &Vec<InputRow>) {
    let lengths: Vec<usize> = vec![2, 4, 3, 7];
    let mut count = 0;
    for row in inputs {
        for output in &row.four_digits {
            if lengths.contains(&output.len()) {
                count += 1;
            }
        }
    }

    println!("Number of 1,4,7,8: {}", count);
}

fn part2(inputs: &Vec<InputRow>) {
    let mut sum = 0;
    for input in inputs {
        sum += decode(input);
    }

    println!("The sum is {}", sum);
}

pub fn solve() {
    let inputs = read_input(INPUT_FILE);
    part1(&inputs);
    part2(&inputs);
}
