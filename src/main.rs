use std::cmp;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./inputs/day1/input.txt").expect("Couldn't read the file");
    println!("Hello, world!");
    let inputs: Vec<u32> = contents.lines().map(|s| s.parse().unwrap()).collect();

    let single_count = count_increased(&inputs);
    let moving_window = moving_window_increased(&inputs, 3);
    println!("Result: {}, {}", single_count, moving_window);
}

fn count_increased(depths: &Vec<u32>) -> u32 {
    let mut count = 0;
    let mut current_depth = &depths[0];

    for depth in depths {
        if depth > current_depth {
            count += 1;
        }
        println!("{}: {}", current_depth, count);
        current_depth = depth;
    }

    count
}

fn moving_window_increased(depths: &Vec<u32>, window_size: usize) -> u32 {
    let mut count = 0;
    let mut current_window_depth = depths[0] + depths[1] + depths[2];

    for index in 0..depths.len() {
        let stop = cmp::min(index + window_size, depths.len());
        let moving_window = depths[index..stop].iter().sum();
        if moving_window > current_window_depth {
            count += 1
        }
        current_window_depth = moving_window;
    }
    count
}
