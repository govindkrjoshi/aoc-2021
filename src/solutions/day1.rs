use std::cmp;
use std::fs;

static INPUT_FILE: &str = "./inputs/day1/input.txt";

fn moving_window_depth(depths: &Vec<u32>, index: usize, window: usize) -> u32 {
    let stop = cmp::min(index + window, depths.len());
    depths[index..stop].iter().sum()
}

fn count_increasing_windows(depths: &Vec<u32>, window: usize) -> u32 {
    let mut count = 0;
    let mut current_depth = moving_window_depth(depths, 0, window);
    for i in 0..depths.len() {
        let window_depth = moving_window_depth(depths, i, window);
        if window_depth > current_depth {
            count += 1;
        }
        current_depth = window_depth;
    }

    count
}

fn read_input(path: &str) -> Vec<u32> {
    let contents = fs::read_to_string(path).expect("Couldn't read the file");
    contents.lines().map(|s| s.parse().unwrap()).collect()
}

pub fn solve() {
    let depths = read_input(INPUT_FILE);
    println!("Part 1: {}", count_increasing_windows(&depths, 1));
    println!("Part 2: {}", count_increasing_windows(&depths, 3));
}

#[cfg(test)]
mod tests {
    static TEST_FILE: &str = "./inputs/day1/test.txt";

    use super::*;

    #[test]
    fn test_for_window_of_size_1() {
        let inputs = read_input(TEST_FILE);
        assert_eq!(7, count_increasing_windows(&inputs, 1));
    }

    #[test]
    fn test_for_window_of_size_3() {
        let inputs = read_input(TEST_FILE);
        assert_eq!(5, count_increasing_windows(&inputs, 3));
    }
}
