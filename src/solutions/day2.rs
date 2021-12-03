extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::fs;

static INPUT_FILE: &str = "./inputs/day2/input.txt";

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

fn command_from_string(cmd: &str, value: &str) -> Command {
    let value = value.parse().unwrap();
    match cmd {
        "forward" => Command::Forward(value),
        "up" => Command::Up(value),
        "down" => Command::Down(value),
        _ => panic!("Invalid command: {}", cmd),
    }
}

fn command_parser(input: &str) -> IResult<&str, Command> {
    let (input, (cmd, value)) = alt((
        separated_pair(tag("forward"), space1, digit1),
        separated_pair(tag("up"), space1, digit1),
        separated_pair(tag("down"), space1, digit1),
    ))(input)?;

    Ok((input, command_from_string(cmd, value)))
}

fn read_input(path: &str) -> Vec<Command> {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    let (_, commands) = separated_list1(newline, command_parser)(&contents).unwrap();

    commands
}

type Interpreter = fn((u32, u32, u32), &Command) -> (u32, u32, u32);

fn interpret_part2((front, depth, aim): (u32, u32, u32), command: &Command) -> (u32, u32, u32) {
    match command {
        Command::Forward(val) => (front + val, depth + aim * val, aim),
        Command::Up(val) => (front, depth, aim - val), // Up decreases the aim
        Command::Down(val) => (front, depth, aim + val),
    }
}

fn interpret_part1((front, depth, aim): (u32, u32, u32), command: &Command) -> (u32, u32, u32) {
    match command {
        Command::Forward(val) => (front + val, depth, aim),
        Command::Up(val) => (front, depth - val, aim), // Going up reduces the depth
        Command::Down(val) => (front, depth + val, aim),
    }
}

fn calculate_position(
    interpreter: Interpreter,
    initial_position: (u32, u32, u32),
    commands: &Vec<Command>,
) -> (u32, u32, u32) {
    let mut current_position = initial_position;

    for command in commands {
        current_position = interpreter(current_position, &command);
    }

    current_position
}

pub fn solve() {
    let commands = read_input(INPUT_FILE);
    let (front, depth, _aim) = calculate_position(interpret_part1, (0, 0, 0), &commands);
    println!("Part 1: {}", front * depth);
    let (front, depth, _aim) = calculate_position(interpret_part2, (0, 0, 0), &commands);
    println!("Part 2: {}", front * depth);
}

#[cfg(test)]
mod tests {
    static TEST_FILE: &str = "./inputs/day2/test.txt";

    use super::*;

    #[test]
    fn can_parse_forward_commands() {
        assert_eq!(command_parser("forward 10"), Ok(("", Command::Forward(10))));
    }

    #[test]
    fn can_parse_up_commands() {
        assert_eq!(command_parser("up 10"), Ok(("", Command::Up(10))));
    }

    #[test]
    fn can_parse_down_commands() {
        assert_eq!(command_parser("down 10"), Ok(("", Command::Down(10))));
    }

    #[test]
    fn can_parse_the_test_file() {
        assert_eq!(
            read_input(TEST_FILE),
            vec![
                Command::Forward(5),
                Command::Down(5),
                Command::Forward(8),
                Command::Up(3),
                Command::Down(8),
                Command::Forward(2),
            ]
        )
    }

    #[test]
    fn can_calculate_the_final_position_part1() {
        let inputs = read_input(TEST_FILE);
        assert_eq!(
            calculate_position(interpret_part1, (0, 0, 0), &inputs),
            (15, 10, 0)
        );
    }

    #[test]
    fn can_calculate_the_final_position_part2() {
        let inputs = read_input(TEST_FILE);
        assert_eq!(
            calculate_position(interpret_part2, (0, 0, 0), &inputs),
            (15, 60, 10)
        );
    }
}
