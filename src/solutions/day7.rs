use nom::{bytes::complete::tag, multi::separated_list0};

static INPUT_FILE: &str = "./inputs/day7/input.txt";
use super::utils::parse_i32;
use std::fs;

fn cost_1(p1: i32, p2: i32) -> i32 {
    (p1 - p2).abs()
}

fn cost_2(p1: i32, p2: i32) -> i32 {
    let diff = (p1 - p2).abs();
    diff * (diff + 1) / 2
}

fn read_input(path: &str) -> Vec<i32> {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    let (_, result) = separated_list0(tag(","), parse_i32)(&contents).unwrap();
    result
}

fn print_minimum_fuel(positions: &Vec<i32>, cost: fn(i32, i32) -> i32) {
    let fuel_to_align_at =
        |at: i32, ps: &Vec<i32>| -> i32 { ps.iter().map(|p| cost(*p, at)).sum() };

    let start = *positions.iter().min().unwrap();
    let end = *positions.iter().max().unwrap();
    let mut min_fuel = i32::MAX;
    let mut min_position = 0;
    for p in start..=end {
        let fuel = fuel_to_align_at(p, positions);
        if fuel < min_fuel {
            min_fuel = fuel;
            min_position = p;
        }

        println!("Fuel required for position {}: {}", p, fuel);
    }
    println!(
        "Minimum fuel required is for position {}: {}",
        min_position, min_fuel
    );
}
pub fn solve() {
    let inputs = read_input(INPUT_FILE);
    print_minimum_fuel(&inputs, cost_1);
    print_minimum_fuel(&inputs, cost_2);
}
